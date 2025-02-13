use validator::Validate;

use crate::{
    domain,
    infrastructure::{self, repository::IRepo},
};

pub struct MlScoreUsecase<'p> {
    pub ml_score_service: domain::services::MlScoreService,
    db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
}

impl<'p> MlScoreUsecase<'p> {
    pub fn new(db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool) -> Self {
        Self {
            ml_score_service: domain::services::MlScoreService,
            db_pool,
        }
    }

    pub async fn set_ml_score(self, ml_score: domain::schemas::MlScoreRequest) -> domain::services::ServiceResult<()> {
        ml_score
            .validate()
            .map_err(|e| domain::services::ServiceError::Validation(e.to_string()))?;

        self.ml_score_service
            .set_ml_score(
                ml_score.client_id,
                ml_score.advertiser_id,
                ml_score.score,
                infrastructure::repository::sqlx_lib::PgScoreRepository::new(self.db_pool),
            )
            .await?;

        Ok(())
    }
}
