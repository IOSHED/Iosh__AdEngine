use validator::Validate;

use crate::{
    domain,
    infrastructure::{self, repository::IRepo},
};

pub struct TimeAdvanceUsecase<'p> {
    campaign_service: domain::services::CampaignService,
    campaign_stat_service: domain::services::CampaignStatService,
    redis_service: domain::services::RedisService<'p>,
    db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
}

impl<'p> TimeAdvanceUsecase<'p> {
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

    pub async fn set_advance(
        &self,
        advance_schema: domain::schemas::TimeAdvanceRequest,
    ) -> domain::services::ServiceResult<domain::schemas::TimeAdvanceResponse> {
        advance_schema
            .validate()
            .map_err(|e| domain::services::ServiceError::Validation(e.to_string()))?;

        self.redis_service.set_advance_time(advance_schema.current_date).await?;

        let old_campaigns = self.redis_service.get_all_active_campaigns().await?;

        for campaign in old_campaigns {
            if campaign.end_date < advance_schema.current_date || advance_schema.current_date < campaign.start_date {
                self.redis_service.del_active_campaigns(&campaign.campaign_id).await?;
            }
        }

        let campaigns = self
            .campaign_service
            .get_active_campaigns(
                advance_schema.current_date,
                infrastructure::repository::sqlx_lib::PgCampaignRepository::new(self.db_pool),
            )
            .await?;

        for campaign in campaigns {
            let (view_clients_id, click_clients_id) = self
                .campaign_stat_service
                .get_or_create_uniq_id(
                    campaign.campaign_id,
                    infrastructure::repository::sqlx_lib::PgCampaignRepository::new(self.db_pool),
                )
                .await?;

            let campaign_active =
                domain::schemas::ActiveCampaignSchema::from((campaign, view_clients_id, click_clients_id));

            self.redis_service.set_active_campaign(campaign_active).await?;
        }

        Ok(domain::schemas::TimeAdvanceResponse {
            current_date: self.redis_service.get_advance_time().await?,
        })
    }
}
