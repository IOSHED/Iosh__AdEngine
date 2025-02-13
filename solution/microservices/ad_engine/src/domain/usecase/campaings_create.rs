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
        domain::validators::validate_campaing_data(
            create_data.start_date,
            create_data.end_date,
            create_data.targeting.age_from,
            create_data.targeting.age_to,
            time_advance,
        )
        .await?;

        self.campaign_service
            .create::<infrastructure::repository::sqlx_lib::PgCampaignRepository>(
                create_data,
                advertiser_id,
                time_advance,
            )
            .await
    }
}
