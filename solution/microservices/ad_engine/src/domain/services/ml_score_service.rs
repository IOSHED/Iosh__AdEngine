use async_trait::async_trait;

use crate::{domain, infrastructure};

#[async_trait]
pub trait ISetMlScore {
    async fn set_ml_score(
        &self,
        client_id: uuid::Uuid,
        advertisers_id: uuid::Uuid,
        score: f64,
    ) -> infrastructure::repository::RepoResult<()>;
}

#[derive(std::fmt::Debug)]
pub struct MlScoreService<'p> {
    db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
}

impl<'p> MlScoreService<'p> {
    pub fn new(db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool) -> Self {
        Self { db_pool }
    }
}

impl<'p> MlScoreService<'p> {
    pub async fn set_ml_score(
        self,
        client_id: uuid::Uuid,
        advertiser_id: uuid::Uuid,
        score: f64,
    ) -> domain::services::ServiceResult<()> {
        infrastructure::repository::sqlx_lib::PgScoreRepository::new(self.db_pool)
            .set_ml_score(client_id, advertiser_id, score)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;
        Ok(())
    }
}
