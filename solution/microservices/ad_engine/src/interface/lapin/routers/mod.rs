//! RabbitMQ message handler implementations for user-related operations.
//!
//! This module provides macro-based implementations of RabbitMQ message
//! handlers for user existence checks and user creation operations.

use crate::{domain, interface};

/// Macro for implementing the RabbitMQ MessageHandler trait.
///
/// This macro reduces boilerplate by generating a standard message handler
/// implementation that deserializes incoming messages, processes them through
/// the specified usecase, and handles response routing.
///
/// # Arguments
/// * `$struct` - The type implementing the MessageHandler trait
/// * `$req_ty` - Request type for deserialization
/// * `$usecase` - Usecase type that processes the request
/// * `$method` - Method name to call on the usecase
macro_rules! impl_rabbit_handler {
    ($struct:ty, $req_ty:ty, $usecase:ty, $method:ident) => {
        #[async_trait::async_trait]
        impl interface::lapin::rabbit_client::MessageHandler for $struct {
            /// Handles incoming RabbitMQ messages by deserializing the payload and
            /// processing it through the appropriate usecase.
            ///
            /// # Arguments
            /// * `context` - RabbitMQ context containing connection details and
            ///   utilities
            /// * `message` - Raw message bytes to be deserialized
            /// * `delivery` - Message delivery information including routing details
            ///
            /// # Returns
            /// * `Result<(), Box<dyn std::error::Error + Send + Sync>>` - Success or
            ///   error result
            async fn handle(
                &self,
                context: &interface::lapin::rabbit_client::RabbitContext,
                message: &[u8],
                delivery: &lapin::message::Delivery,
            ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                let request: $req_ty = serde_json::from_slice(message)?;
                let result = <$usecase>::new().$method(request, &context.db_pool.clone()).await;

                match result {
                    Ok(response) =>
                        context
                            .send_response(&delivery.routing_key.as_str(), &response)
                            .await?,
                    Err(e) =>
                        context
                            .send_response(delivery.routing_key.as_str(), &e.to_string())
                            .await?,
                }

                Ok(())
            }

            /// Creates a boxed clone of this message handler.
            ///
            /// # Returns
            /// * Box<dyn MessageHandler> - Boxed clone of the handler
            fn boxed_clone(&self) -> Box<dyn interface::lapin::rabbit_client::MessageHandler + Send + Sync> {
                Box::new(Self)
            }
        }
    };
}

/// Message handler for checking if a user exists.
pub struct UserAreExist;
impl_rabbit_handler!(
    UserAreExist,
    domain::schemas::UserAreExistRequest,
    domain::usecase::UserAreExistUsecase,
    are_exist
);

/// Message handler for creating new users.
pub struct UserCreate;
impl_rabbit_handler!(
    UserCreate,
    domain::schemas::RegisterRequest,
    domain::usecase::UserRegisterUsecase,
    register
);
