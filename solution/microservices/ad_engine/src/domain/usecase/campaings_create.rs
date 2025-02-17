use validator::Validate;

use crate::{
    domain,
    infrastructure::{self, repository::IRepo},
};

pub struct CampaignsCreateUsecase<'p> {
    campaign_service: domain::services::CampaignService,
    moderate_text_service: domain::services::ModerateTextService,
    redis_service: domain::services::RedisService<'p>,
    db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
    redis_pool: &'p infrastructure::database_connection::redis::RedisPool,
}

impl<'p> CampaignsCreateUsecase<'p> {
    pub fn new(
        db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
        redis_pool: &'p infrastructure::database_connection::redis::RedisPool,
        app_state: &'p domain::configurate::AppState,
    ) -> Self {
        Self {
            campaign_service: domain::services::CampaignService,
            moderate_text_service: domain::services::ModerateTextService::new(app_state.auto_moderating_sensitivity),
            redis_service: domain::services::RedisService::new(redis_pool),
            db_pool,
            redis_pool,
        }
    }

    pub async fn create(
        self,
        create_data: domain::schemas::CampaignsCreateRequest,
        advertiser_id: uuid::Uuid,
    ) -> domain::services::ServiceResult<domain::schemas::CampaignSchema> {
        let time_advance: u32 = self.redis_service.get_advance_time().await?;

        create_data.validate()?;
        domain::validators::validate_campaign_data(
            create_data.start_date,
            create_data.end_date,
            create_data.targeting.age_from,
            create_data.targeting.age_to,
            create_data.impressions_limit,
            create_data.clicks_limit,
            time_advance,
        )
        .await?;

        self.moderate_text_service
            .check_abusive_content(
                &[create_data.ad_text.clone(), create_data.ad_title.clone()],
                self.redis_service.get_is_activate_auto_moderate().await?,
                infrastructure::repository::redis::RedisObsceneWordRepository::new(self.redis_pool, self.db_pool),
            )
            .await?;

        let campaign = self
            .campaign_service
            .create::<infrastructure::repository::sqlx_lib::PgCampaignRepository>(
                create_data,
                advertiser_id,
                infrastructure::repository::sqlx_lib::PgCampaignRepository::new(self.db_pool),
            )
            .await?;

        let advanced_time = self.redis_service.get_advance_time().await.unwrap_or(0);
        if advanced_time <= campaign.end_date && advanced_time >= campaign.start_date {
            let active_campaign = domain::schemas::ActiveCampaignSchema::from((campaign.clone(), vec![], vec![]));
            self.redis_service.set_active_campaign(active_campaign).await?;
        }

        domain::services::PrometheusService::increment_campaign_created(advanced_time);

        Ok(campaign)
    }
}
