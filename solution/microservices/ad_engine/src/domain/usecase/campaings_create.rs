use validator::Validate;

use crate::{
    domain,
    infrastructure::{self, repository::IRepo},
};

pub struct CampaignsCreateUsecase<'p> {
    campaign_service: domain::services::CampaignService,
    redis_service: domain::services::RedisService<'p>,
    db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
}

impl<'p> CampaignsCreateUsecase<'p> {
    pub fn new(
        db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
        redis_pool: &'p infrastructure::database_connection::redis::RedisPool,
    ) -> Self {
        Self {
            campaign_service: domain::services::CampaignService,
            redis_service: domain::services::RedisService::new(redis_pool),
            db_pool,
        }
    }

    pub async fn create(
        self,
        create_data: domain::schemas::CampaignsCreateRequest,
        advertiser_id: uuid::Uuid,
    ) -> domain::services::ServiceResult<domain::schemas::CampaignSchema> {
        let time_advance: u32 = self.redis_service.get_advance_time().await?;

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
                infrastructure::repository::sqlx_lib::PgCampaignRepository::new(self.db_pool),
            )
            .await
    }
}
