use validator::Validate;

use crate::{domain, infrastructure};

pub struct ModerateSetSettingsUsecase<'p> {
    redis_service: domain::services::RedisService<'p>,
}

impl<'p> ModerateSetSettingsUsecase<'p> {
    pub fn new(redis_pool: &'p infrastructure::database_connection::redis::RedisPool) -> Self {
        Self {
            redis_service: domain::services::RedisService::new(redis_pool),
        }
    }

    pub async fn set(
        &self,
        moderate_schema: domain::schemas::ModerateSchema,
    ) -> domain::services::ServiceResult<domain::schemas::ModerateSchema> {
        moderate_schema
            .validate()
            .map_err(|e| domain::services::ServiceError::Validation(e.to_string()))?;

        self.redis_service
            .set_is_activate_auto_moderate(moderate_schema.is_activate)
            .await?;

        Ok(domain::schemas::ModerateSchema {
            is_activate: self.redis_service.get_is_activate_auto_moderate().await?,
        })
    }
}
