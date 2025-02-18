use async_trait::async_trait;

use crate::{domain, infrastructure};

/// Defines the interface for adding words to the moderation list
///
/// This trait provides an asynchronous method to add new words to the
/// moderation system. Implementations should handle the persistence of the
/// added words.
#[async_trait]
pub trait IAddModerateList {
    /// Adds a list of words to the moderation system
    ///
    /// # Arguments
    /// * `add_words` - Vector of strings containing words to be added to
    ///   moderation list
    ///
    /// # Returns
    /// * `RepoResult<()>` - Repository result indicating success or failure
    async fn add_list(&self, add_words: Vec<String>) -> infrastructure::repository::RepoResult<()>;
}

/// Defines the interface for removing words from the moderation list
///
/// This trait provides an asynchronous method to delete existing words from the
/// moderation system. Implementations should handle the removal of words from
/// persistence.
#[async_trait]
pub trait IDeleteModerateList {
    /// Deletes a list of words from the moderation system
    ///
    /// # Arguments
    /// * `delete_words` - Vector of strings containing words to be removed from
    ///   moderation list
    ///
    /// # Returns
    /// * `RepoResult<()>` - Repository result indicating success or failure
    async fn delete_list(&self, delete_words: Vec<String>) -> infrastructure::repository::RepoResult<()>;
}

/// Service implementation for managing the moderation list
///
/// Provides methods to add and delete words from the moderation system
/// while abstracting the underlying repository implementation.
#[derive(std::fmt::Debug)]
pub struct ModerateListService;

impl<'p> ModerateListService {
    /// Deletes specified words from the moderation list using the provided
    /// repository
    ///
    /// # Arguments
    /// * `delete_words` - Vector of strings to be deleted
    /// * `repo` - Repository implementation that satisfies IDeleteModerateList
    ///   trait
    ///
    /// # Returns
    /// * `ServiceResult<()>` - Result indicating success or wrapped service
    ///   error
    pub async fn delete_list<R: IDeleteModerateList>(
        &self,
        delete_words: Vec<String>,
        repo: R,
    ) -> domain::services::ServiceResult<()> {
        repo.delete_list(delete_words)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))
    }

    /// Adds specified words to the moderation list using the provided
    /// repository
    ///
    /// # Arguments
    /// * `delete_words` - Vector of strings to be added
    /// * `repo` - Repository implementation that satisfies IAddModerateList
    ///   trait
    ///
    /// # Returns
    /// * `ServiceResult<()>` - Result indicating success or wrapped service
    ///   error
    pub async fn add_list<R: IAddModerateList>(
        &self,
        delete_words: Vec<String>,
        repo: R,
    ) -> domain::services::ServiceResult<()> {
        repo.add_list(delete_words)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))
    }
}
