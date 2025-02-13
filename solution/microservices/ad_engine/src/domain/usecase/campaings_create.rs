use validator::Validate;

use crate::{domain, infrastructure};

pub struct CampaignsCreateUsecase<'p> {
    campaign_service: domain::services::CampaignService<'p>,
    redis_service: domain::services::RedisService<'p>,
}

impl<'p> CampaignsCreateUsecase<'p> {
    pub fn new(
        db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
        redis_pool: &'p infrastructure::database_connection::redis::RedisPool,
    ) -> Self {
        Self {
            campaign_service: domain::services::CampaignService::new(db_pool),
            redis_service: domain::services::RedisService::new(redis_pool),
        }
    }

    pub async fn create(
        self,
        create_data: domain::schemas::CampaignsCreateRequest,
        advertiser_id: uuid::Uuid,
    ) -> domain::services::ServiceResult<domain::schemas::CampaignSchema> {
        let time_advance: u32 = self.redis_service.get("time_advance").await?;

        create_data.validate()?;
        (create_data.start_date >= time_advance)
            .then_some(())
            .ok_or(domain::services::ServiceError::Validation(
                "start_data must be more or equal time_advance".into(),
            ))?;
        (create_data.start_date <= create_data.end_date).then_some(()).ok_or(
            domain::services::ServiceError::Validation("start_data must be under or equal end_date".into()),
        )?;
        (create_data.targeting.age_from <= create_data.targeting.age_to)
            .then_some(())
            .ok_or(domain::services::ServiceError::Validation(
                "age_from must be under or equal age_to".into(),
            ))?;

        self.campaign_service
            .create(create_data, advertiser_id, time_advance)
            .await
    }
}
