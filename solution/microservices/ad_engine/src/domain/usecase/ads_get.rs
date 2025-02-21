use crate::{
    domain,
    infrastructure::{self, repository::IRepo},
};

pub struct AdsGetUsecase<'p> {
    ads_service: domain::services::AdsService,
    campaign_stat_service: domain::services::CampaignStatService,
    redis_service: domain::services::RedisService<'p>,
    db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
    redis_pool: &'p infrastructure::database_connection::redis::RedisPool,
    moderate_text_service: domain::services::ModerateTextService,
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
                app_state.ads_range_between_non_unique_and_unique_campaign,
            ),
            moderate_text_service: domain::services::ModerateTextService::new(app_state.auto_moderating_sensitivity),
            campaign_stat_service: domain::services::CampaignStatService,
            redis_service: domain::services::RedisService::new(redis_pool),
            db_pool,
            redis_pool,
        }
    }

    pub async fn execute(&self, client_id: uuid::Uuid) -> domain::services::ServiceResult<domain::schemas::AdSchema> {
        let active_campaigns = self.redis_service.get_all_active_campaigns().await?;
        let advanced_time = self.redis_service.get_advance_time().await?;

        let mut ads = self
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

        if !campaign.view_clients_id.contains(&client_id) {
            self.campaign_stat_service
                .view_campaign(
                    ads.ad_id,
                    client_id,
                    campaign.cost_per_impression,
                    advanced_time,
                    infrastructure::repository::sqlx_lib::PgCampaignRepository::new(self.db_pool),
                )
                .await?;

            campaign.view_clients_id.push(client_id);
            domain::services::PrometheusService::ads_visits(advanced_time, campaign.cost_per_impression);

            self.redis_service.set_active_campaign(campaign).await?;
        }

        let new_texts = self
            .moderate_text_service
            .hide_abusive_content(
                &[ads.ad_text, ads.ad_title],
                self.redis_service.get_is_activate_auto_moderate().await?,
                infrastructure::repository::redis::RedisObsceneWordRepository::new(self.redis_pool, self.db_pool),
            )
            .await?;

        ads.ad_text = new_texts[0].clone();
        ads.ad_title = new_texts[1].clone();

        Ok(ads)
    }
}
