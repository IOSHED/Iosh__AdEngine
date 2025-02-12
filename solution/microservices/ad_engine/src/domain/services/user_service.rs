use async_trait::async_trait;

use crate::{domain, infrastructure};

/// Defines the interface for registering a new user in the system.
///
/// This trait provides the contract for user registration functionality,
/// handling all required user data fields and returning the created user.
#[async_trait]
pub trait IRegisterUser {
    /// Registers a new user with the provided information.
    ///
    /// # Arguments
    /// * `telegram_id` - Unique Telegram identifier for the user
    /// * `birth_day` - User's date of birth as NaiveDate
    /// * `latitude` - Geographic latitude of user's location
    /// * `longitude` - Geographic longitude of user's location
    /// * `city` - Name of user's city
    /// * `country_code` - Two-letter country code (ISO 3166-1 alpha-2)
    /// * `bio` - Optional user biography/description
    /// * `interests` - List of user's interests/hobbies
    ///
    /// # Returns
    /// * `RepoResult<UserReturningSchema>` - Result containing the created user
    ///   data or a repository error
    async fn register(
        &self,
        telegram_id: i32,
        birth_day: chrono::NaiveDate,
        latitude: f64,
        longitude: f64,
        city: String,
        country_code: String,
        bio: Option<String>,
        interests: Vec<String>,
    ) -> infrastructure::repository::RepoResult<infrastructure::repository::sqlx_lib::UserReturningSchema>;
}

/// Defines the interface for checking user existence in the system.
#[async_trait]
pub trait IUserAreExists {
    /// Checks if a user with the given Telegram ID exists.
    ///
    /// # Arguments
    /// * `telegram_id` - Telegram identifier to check
    ///
    /// # Returns
    /// * `RepoResult<bool>` - Result containing true if user exists, false
    ///   otherwise
    async fn are_exist(&self, telegram_id: i32) -> infrastructure::repository::RepoResult<bool>;
}

/// Service layer implementation for user-related operations.
///
/// Provides high-level business logic for user management, including
/// registration and existence checks.
#[derive(std::fmt::Debug)]
pub struct UserService;

impl UserService {
    /// Registers a new user in the system using the provided repository
    /// implementation.
    ///
    /// # Arguments
    /// * `register_data` - Structured data containing all required user
    ///   information
    /// * `repo` - Repository implementation that satisfies the IRegisterUser
    ///   trait
    ///
    /// # Returns
    /// * `ServiceResult<UserProfileSchema>` - Result containing the created
    ///   user profile or a service error
    ///
    /// # Errors
    /// Returns ServiceError::Unknown if:
    /// * Birth date parsing fails
    /// * City is missing
    /// * Country code is missing
    #[tracing::instrument(name = "`UserService` register user")]
    pub async fn register<R: domain::services::repository::IRegisterUser + std::fmt::Debug>(
        self,
        register_data: domain::schemas::RegisterRequest,
        repo: R,
    ) -> domain::services::ServiceResult<domain::schemas::UserProfileSchema> {
        let repo_user = repo
            .register(
                register_data.telegram_id as i32,
                chrono::NaiveDate::parse_from_str(&register_data.birth_day, "%Y-%m-%d")
                    .map_err(|_| domain::services::ServiceError::Unknown)?,
                register_data.latitude,
                register_data.longitude,
                register_data
                    .city
                    .ok_or_else(|| domain::services::ServiceError::Unknown)?,
                register_data
                    .country_code
                    .ok_or_else(|| domain::services::ServiceError::Unknown)?,
                register_data.bio,
                register_data.interests,
            )
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;

        Ok(repo_user.into())
    }

    /// Checks if a user exists in the system by their Telegram ID.
    ///
    /// # Arguments
    /// * `telegram_id` - Telegram identifier to check
    /// * `repo` - Repository implementation that satisfies the IUserAreExists
    ///   trait
    ///
    /// # Returns
    /// * `ServiceResult<bool>` - Result containing existence status or a
    ///   service error
    #[tracing::instrument(name = "`UserService` ask are exists user")]
    pub async fn are_exist<R: domain::services::repository::IUserAreExists + std::fmt::Debug>(
        self,
        telegram_id: usize,
        repo: R,
    ) -> domain::services::ServiceResult<bool> {
        let repo_user = repo
            .are_exist(telegram_id as i32)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;

        Ok(repo_user.into())
    }
}

/// Implements conversion from repository user schema to domain user profile
/// schema.
impl From<infrastructure::repository::sqlx_lib::UserReturningSchema> for domain::schemas::UserProfileSchema {
    fn from(user: infrastructure::repository::sqlx_lib::UserReturningSchema) -> Self {
        Self {
            telegram_id: user.telegram_id as usize,
            birth_day: user.birth_day.to_string(),
            city: user.city,
            bio: user.bio,
            interests: user.interests,
            country_code: user.country_code,
        }
    }
}
