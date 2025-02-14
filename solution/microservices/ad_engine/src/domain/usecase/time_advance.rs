use validator::Validate;

use crate::{domain, infrastructure};

pub struct TimeAdvanceUsecase<'p> {
    redis_service: domain::services::RedisService<'p>,
}

impl<'p> TimeAdvanceUsecase<'p> {
    pub fn new(redis_pool: &'p infrastructure::database_connection::redis::RedisPool) -> Self {
        Self {
            redis_service: domain::services::RedisService::new(redis_pool),
        }
    }

    pub async fn set_advance(
        &self,
        advance_schema: domain::schemas::TimeAdvanceRequest,
    ) -> domain::services::ServiceResult<domain::schemas::TimeAdvanceResponse> {
        advance_schema
            .validate()
            .map_err(|e| domain::services::ServiceError::Validation(e.to_string()))?;

        self.redis_service.set_advance_time(advance_schema.current_date).await?;
        Ok(domain::schemas::TimeAdvanceResponse {
            current_date: self.redis_service.get_advance_time().await?,
        })
    }
}
