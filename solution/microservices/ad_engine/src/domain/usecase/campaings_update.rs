use validator::Validate;

use crate::{domain, infrastructure};

pub struct CampaignsUpdateUsecase<'p> {
    campaign_service: domain::services::CampaignService<'p>,
    redis_service: domain::services::RedisService<'p>,
}

impl<'p> CampaignsUpdateUsecase<'p> {
    pub fn new(
        db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
        redis_pool: &'p infrastructure::database_connection::redis::RedisPool,
    ) -> Self {
        Self {
            campaign_service: domain::services::CampaignService::new(db_pool),
            redis_service: domain::services::RedisService::new(redis_pool),
        }
    }

    pub async fn update(
        self,
        update_data: domain::schemas::CampaignsUpdateRequest,
        advertiser_id: uuid::Uuid,
        campaign_id: uuid::Uuid,
    ) -> domain::services::ServiceResult<domain::schemas::CampaignSchema> {
        let time_advance: u32 = self.redis_service.get("time_advance").await?;

        update_data.validate()?;
        (update_data.targeting.age_from <= update_data.targeting.age_to)
            .then_some(())
            .ok_or(domain::services::ServiceError::Validation(
                "age_from must be under or equal age_to".into(),
            ))?;

        self.campaign_service
            .update(update_data, advertiser_id, campaign_id, time_advance)
            .await
    }
}
