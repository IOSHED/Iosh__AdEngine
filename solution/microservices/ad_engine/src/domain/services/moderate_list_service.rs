use async_trait::async_trait;

use crate::{domain, infrastructure};

#[async_trait]
pub trait IAddModerateList {
    async fn add_list(&self, add_words: Vec<String>) -> infrastructure::repository::RepoResult<()>;
}

#[async_trait]
pub trait IDeleteModerateList {
    async fn delete_list(&self, delete_words: Vec<String>) -> infrastructure::repository::RepoResult<()>;
}

#[derive(std::fmt::Debug)]
pub struct ModerateListService;

impl<'p> ModerateListService {
    pub async fn delete_list<R: IDeleteModerateList>(
        &self,
        delete_words: Vec<String>,
        repo: R,
    ) -> domain::services::ServiceResult<()> {
        repo.delete_list(delete_words)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))
    }

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
