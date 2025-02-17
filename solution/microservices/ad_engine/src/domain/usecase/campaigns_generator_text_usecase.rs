use validator::Validate;

use crate::{
    domain,
    infrastructure::{self, repository::IRepo},
};

pub struct CampaignsGeneratorTextUsecase<'p> {
    yandex_gpt_service: domain::services::YandexGptService,
    campaign_service: domain::services::CampaignService,
    redis_service: domain::services::RedisService<'p>,
    db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
}

impl<'p> CampaignsGeneratorTextUsecase<'p> {
    pub fn new(
        db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
        redis_pool: &'p infrastructure::database_connection::redis::RedisPool,
        app_state: &'p domain::configurate::AppState,
    ) -> Self {
        Self {
            yandex_gpt_service: domain::services::YandexGptService::new(
                app_state.yandex_folder_id.clone(),
                app_state.yandex_api_key.clone(),
                app_state.gpt_temperature,
                app_state.gpt_max_tokens,
                app_state.system_prompt_for_generate_title.clone(),
                app_state.system_prompt_for_generate_body.clone(),
            ),
            campaign_service: domain::services::CampaignService,
            redis_service: domain::services::RedisService::new(redis_pool),
            db_pool,
        }
    }

    pub async fn generate(
        &self,
        generate_schema: domain::schemas::CampaignsGenerateTextRequest,
        advertiser_id: uuid::Uuid,
        campaign_id: uuid::Uuid,
    ) -> domain::services::ServiceResult<domain::schemas::CampaignSchema> {
        generate_schema.validate()?;

        let mut campaign = self
            .campaign_service
            .get_by_id(
                advertiser_id,
                campaign_id,
                infrastructure::repository::sqlx_lib::PgCampaignRepository::new(self.db_pool),
            )
            .await?;

        self.yandex_gpt_service
            .generate_text_for_campaign(&mut campaign, generate_schema)
            .await?;

        let advanced_time = self.redis_service.get_advance_time().await?;
        if advanced_time <= campaign.end_date && advanced_time >= campaign.start_date {
            let mut active_campaign = self.redis_service.get_active_campaign(&campaign_id).await?;
            active_campaign.ad_text = campaign.ad_text.clone();
            active_campaign.ad_title = campaign.ad_title.clone();
            self.redis_service.set_active_campaign(active_campaign).await?;
        }

        let campaign = self
            .campaign_service
            .update(
                domain::schemas::CampaignsUpdateRequest::from(campaign),
                advertiser_id,
                campaign_id,
                advanced_time,
                infrastructure::repository::sqlx_lib::PgCampaignRepository::new(self.db_pool),
            )
            .await?;

        domain::services::PrometheusService::increment_campaign_updated(advanced_time);

        Ok(campaign)
    }
}
