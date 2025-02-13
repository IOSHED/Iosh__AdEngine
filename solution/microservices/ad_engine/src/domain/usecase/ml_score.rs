use validator::Validate;

use crate::{domain, infrastructure};

pub struct MlScoreUsecase<'p> {
    pub ml_score_service: domain::services::MlScoreService<'p>,
}

impl<'p> MlScoreUsecase<'p> {
    pub fn new(db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool) -> Self {
        Self {
            ml_score_service: domain::services::MlScoreService::new(db_pool),
        }
    }

    pub async fn set_ml_score(self, ml_score: domain::schemas::MlScoreRequest) -> domain::services::ServiceResult<()> {
        ml_score
            .validate()
            .map_err(|e| domain::services::ServiceError::Validation(e.to_string()))?;

        self.ml_score_service
            .set_ml_score::<infrastructure::repository::sqlx_lib::PgScoreRepository>(
                ml_score.client_id,
                ml_score.advertiser_id,
                ml_score.score,
            )
            .await?;

        Ok(())
    }
}
