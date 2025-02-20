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

    /// Retrieves the list of moderated words from the repository
    ///
    /// # Arguments
    /// * `repo` - Repository implementation that satisfies IGetAbusiveWords
    ///   trait
    ///
    /// # Returns
    /// * `ServiceResult<Vec<String>>` - Result containing vector of moderated
    ///   words or wrapped service error
    pub async fn get_list<R: domain::services::repository::IGetAbusiveWords>(
        &self,
        repo: R,
    ) -> domain::services::ServiceResult<Vec<String>> {
        repo.get_words()
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    use super::*;

    struct MockAddModerateListRepo {
        result: Result<(), infrastructure::repository::RepoError>,
    }

    #[async_trait]
    impl IAddModerateList for MockAddModerateListRepo {
        async fn add_list(&self, _add_words: Vec<String>) -> infrastructure::repository::RepoResult<()> {
            self.result.clone()
        }
    }

    struct MockDeleteModerateListRepo {
        result: Result<(), infrastructure::repository::RepoError>,
    }

    #[async_trait]
    impl IDeleteModerateList for MockDeleteModerateListRepo {
        async fn delete_list(&self, _delete_words: Vec<String>) -> infrastructure::repository::RepoResult<()> {
            self.result.clone()
        }
    }

    #[tokio::test]
    async fn test_add_list_success() {
        let mock_repo = MockAddModerateListRepo { result: Ok(()) };
        let service = ModerateListService;
        let words_to_add = vec!["word1".into(), "word2".into()];

        let result = service.add_list(words_to_add.clone(), mock_repo).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_add_list_failure() {
        let mock_repo = MockAddModerateListRepo {
            result: Err(infrastructure::repository::RepoError::Unknown),
        };
        let service = ModerateListService;
        let words_to_add = vec!["word1".into(), "word2".into()];

        let result = service.add_list(words_to_add.clone(), mock_repo).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_list_success() {
        let mock_repo = MockDeleteModerateListRepo { result: Ok(()) };
        let service = ModerateListService;
        let words_to_delete = vec!["word1".into(), "word2".into()];

        let result = service.delete_list(words_to_delete.clone(), mock_repo).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_list_failure() {
        let mock_repo = MockDeleteModerateListRepo {
            result: Err(infrastructure::repository::RepoError::Unknown),
        };
        let service = ModerateListService;
        let words_to_delete = vec!["word1".into(), "word2".into()];

        let result = service.delete_list(words_to_delete.clone(), mock_repo).await;

        assert!(result.is_err());
    }
}
