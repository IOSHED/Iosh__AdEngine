use async_trait::async_trait;

use crate::interface;

/// A trait for handling messages in an asynchronous message processing system.
/// Implementors must be both Send and Sync to ensure thread safety.
#[async_trait]
pub trait MessageHandler: Send + Sync {
    /// Handles an incoming message asynchronously.
    ///
    /// # Arguments
    /// * `context` - The RabbitMQ context containing connection details and
    ///   state
    /// * `message` - The raw message bytes to process
    /// * `delivery` - Metadata about the message delivery
    ///
    /// # Returns
    /// * `Result<(), Box<dyn std::error::Error + Send + Sync>>` - Ok(()) on
    ///   success, or an error if handling fails
    async fn handle(
        &self,
        context: &interface::lapin::rabbit_client::RabbitContext,
        message: &[u8],
        delivery: &lapin::message::Delivery,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// Creates a boxed clone of this message handler.
    ///
    /// # Returns
    /// * `Box<dyn MessageHandler + Send + Sync>` - A heap-allocated clone of
    ///   this handler
    fn boxed_clone(&self) -> Box<dyn MessageHandler + Send + Sync>;
}

/// Represents a message routing rule that maps a pattern to a handler.
pub struct Route {
    /// The handler to process messages that match the pattern
    pub handler: Box<dyn MessageHandler + Send + Sync>,
}

impl Route {
    /// Creates a new Route with the given handler.
    ///
    /// # Arguments
    /// * `handler` - The message handler implementation
    ///
    /// # Returns
    /// * `Route` - A new Route instance
    pub fn new(handler: impl MessageHandler + Send + Sync + 'static) -> Self {
        Self {
            handler: Box::new(handler),
        }
    }
}
