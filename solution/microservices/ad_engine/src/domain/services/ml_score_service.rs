use async_trait::async_trait;

use crate::{domain, infrastructure};

#[async_trait]
pub trait ISetMlScore {
    async fn set_ml_score(
        &self,
        client_id: uuid::Uuid,
        advertiser_id: uuid::Uuid,
        score: f64,
    ) -> infrastructure::repository::RepoResult<()>;
}

#[derive(std::fmt::Debug)]
pub struct MlScoreService;

impl<'p> MlScoreService {
    #[tracing::instrument(name = "`MlScoreService` set ML score", skip(repo))]
    pub async fn set_ml_score<R: infrastructure::repository::IRepo<'p> + ISetMlScore>(
        &self,
        client_id: uuid::Uuid,
        advertiser_id: uuid::Uuid,
        score: f64,
        repo: R,
    ) -> domain::services::ServiceResult<()> {
        repo.set_ml_score(client_id, advertiser_id, score)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;
        Ok(())
    }
}
