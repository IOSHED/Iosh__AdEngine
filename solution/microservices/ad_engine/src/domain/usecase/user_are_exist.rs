/// Module for handling user existence verification use cases
use crate::{domain, infrastructure};

/// Usecase for checking if a user exists in the system
///
/// This struct encapsulates the business logic for verifying user existence
/// by delegating to the underlying user service layer.
pub struct UserAreExistUsecase {
    user_service: domain::services::UserService,
}

impl UserAreExistUsecase {
    /// Creates a new instance of UserAreExistUsecase
    ///
    /// # Returns
    /// * `Self` - A new UserAreExistUsecase instance with initialized user
    ///   service
    pub fn new() -> Self {
        Self {
            user_service: domain::services::UserService,
        }
    }

    /// Checks if a user exists based on their Telegram ID
    ///
    /// # Arguments
    /// * `user_schema` - The request schema containing the Telegram ID to check
    /// * `db_pool` - Database connection pool for executing the query
    ///
    /// # Returns
    /// * `ServiceResult<bool>` - Returns true if user exists, false otherwise,
    ///   wrapped in a ServiceResult
    ///
    /// # Errors
    /// * Returns a ServiceError if the database operation fails
    pub async fn are_exist(
        self,
        user_schema: domain::schemas::UserAreExistRequest,
        db_pool: &infrastructure::database_connection::sqlx_lib::SqlxPool,
    ) -> domain::services::ServiceResult<bool> {
        self.user_service
            .are_exist(
                user_schema.telegram_id,
                infrastructure::repository::sqlx_lib::PgUserRepository::new(db_pool),
            )
            .await
    }
}
