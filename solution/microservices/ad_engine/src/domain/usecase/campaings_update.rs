use validator::Validate;

use crate::{
    domain,
    infrastructure::{self, repository::IRepo},
};

pub struct CampaignsUpdateUsecase<'p> {
    campaign_service: domain::services::CampaignService,
    redis_service: domain::services::RedisService<'p>,
    db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
}

impl<'p> CampaignsUpdateUsecase<'p> {
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

    pub async fn update(
        self,
        update_data: domain::schemas::CampaignsUpdateRequest,
        advertiser_id: uuid::Uuid,
        campaign_id: uuid::Uuid,
    ) -> domain::services::ServiceResult<domain::schemas::CampaignSchema> {
        let time_advance: u32 = self.redis_service.get_advance_time().await?;

        update_data.validate()?;
        domain::validators::validate_campaing_data(
            update_data.start_date,
            update_data.end_date,
            update_data.targeting.age_from,
            update_data.targeting.age_to,
            time_advance,
        )
        .await?;

        self.campaign_service
            .update(
                update_data,
                advertiser_id,
                campaign_id,
                time_advance,
                infrastructure::repository::sqlx_lib::PgCampaignRepository::new(self.db_pool),
            )
            .await
    }
}
