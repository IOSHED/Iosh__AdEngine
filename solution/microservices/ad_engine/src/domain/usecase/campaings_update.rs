use validator::Validate;

use crate::{
    domain,
    infrastructure::{self, repository::IRepo},
};

pub struct CampaignsUpdateUsecase<'p> {
    campaign_service: domain::services::CampaignService,
    campaign_stat_service: domain::services::CampaignStatService,
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
            campaign_stat_service: domain::services::CampaignStatService,
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
        domain::validators::validate_campaign_data(
            update_data.start_date,
            update_data.end_date,
            update_data.targeting.age_from,
            update_data.targeting.age_to,
            update_data.impressions_limit,
            update_data.clicks_limit,
            time_advance,
        )
        .await?;

        let campaign = self
            .campaign_service
            .update(
                update_data,
                advertiser_id,
                campaign_id,
                time_advance,
                infrastructure::repository::sqlx_lib::PgCampaignRepository::new(self.db_pool),
            )
            .await?;

        let advanced_time = self.redis_service.get_advance_time().await.unwrap_or(0);
        if advanced_time <= campaign.end_date && advanced_time >= campaign.start_date {
            let (view_clients_id, click_clients_id) = self
                .campaign_stat_service
                .get_or_create_uniq_id(
                    campaign.campaign_id,
                    infrastructure::repository::sqlx_lib::PgCampaignRepository::new(self.db_pool),
                )
                .await?;

            let active_campaign =
                domain::schemas::ActiveCampaignSchema::from((campaign.clone(), view_clients_id, click_clients_id));
            self.redis_service.set_active_campaign(active_campaign).await?;
        }

        Ok(campaign)
    }
}
