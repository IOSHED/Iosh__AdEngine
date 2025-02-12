use validator::Validate;

use crate::{domain, infrastructure};

/// A use case for handling user registration in the system.
/// This struct coordinates the registration process by managing user and
/// location services.
pub struct UserRegisterUsecase {
    user_service: domain::services::UserService,
}

impl UserRegisterUsecase {
    /// Creates a new instance of UserRegisterUsecase with default services.
    ///
    /// # Returns
    /// * `Self` - A new UserRegisterUsecase instance
    pub fn new() -> Self {
        Self {
            user_service: domain::services::UserService,
        }
    }

    /// Handles the user registration process.
    ///
    /// This method performs the following steps:
    /// 1. Validates the registration data
    /// 2. Enriches location data if city/country are missing
    /// 3. Registers the user via the user service
    ///
    /// # Arguments
    /// * `register_data` - The registration request data containing user
    ///   information
    /// * `db_pool` - Database connection pool for persistence
    ///
    /// # Returns
    /// * `ServiceResult<RegisterResponse>` - Result containing the registered
    ///   user profile or error
    ///
    /// # Errors
    /// * Returns `ServiceError` if validation fails or registration cannot be
    ///   completed
    pub async fn register(
        self,
        register_data: domain::schemas::RegisterRequest,
        db_pool: &infrastructure::database_connection::sqlx_lib::SqlxPool,
    ) -> domain::services::ServiceResult<domain::schemas::RegisterResponse> {
        register_data
            .validate()
            .map_err::<domain::services::ServiceError, _>(|e| e.into())?;

        let profile = self
            .user_service
            .register(
                register_data,
                infrastructure::repository::sqlx_lib::PgUserRepository::new(db_pool),
            )
            .await?;

        Ok(domain::schemas::RegisterResponse { profile })
    }
}
