use crate::{
    domain,
    infrastructure::{self, repository::IRepo},
};

pub struct AdsGetUsecase<'p> {
    ads_service: domain::services::AdsService,
    campaign_stat_service: domain::services::CampaignStatService,
    redis_service: domain::services::RedisService<'p>,
    db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
}

impl<'p> AdsGetUsecase<'p> {
    pub fn new(
        db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
        redis_pool: &'p infrastructure::database_connection::redis::RedisPool,
        app_state: &domain::configurate::AppState,
    ) -> Self {
        Self {
            ads_service: domain::services::AdsService::new(
                app_state.ads_weight_profit,
                app_state.ads_weight_relevance,
                app_state.ads_weight_fulfillment,
                app_state.ads_weight_time_left,
            ),
            campaign_stat_service: domain::services::CampaignStatService,
            redis_service: domain::services::RedisService::new(redis_pool),
            db_pool,
        }
    }

    pub async fn execute(&self, client_id: uuid::Uuid) -> domain::services::ServiceResult<domain::schemas::AdSchema> {
        let active_campaigns = self.redis_service.get_all_active_campaigns().await?;
        let advanced_time = self.redis_service.get_advance_time().await?;

        let ads = self
            .ads_service
            .recommendation_ads(
                active_campaigns,
                client_id,
                advanced_time,
                infrastructure::repository::sqlx_lib::PgClientRepository::new(self.db_pool),
                infrastructure::repository::sqlx_lib::PgScoreRepository::new(self.db_pool),
            )
            .await?;

        let mut campaign = self.redis_service.get_active_campaign(&ads.ad_id).await?;

        self.campaign_stat_service
            .view_campaign(
                ads.ad_id,
                client_id,
                campaign.cost_per_impressions,
                advanced_time,
                infrastructure::repository::sqlx_lib::PgCampaignRepository::new(self.db_pool),
            )
            .await?;

        campaign.view_clients_id.push(client_id);
        self.redis_service.set_active_campaign(campaign).await?;

        domain::services::PrometheusService::increment_ads_visits(advanced_time);

        Ok(ads)
    }
}
