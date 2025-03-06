use crate::{
    domain,
    infrastructure::{self, repository::IRepo},
};

pub struct AdsClickUsecase<'p> {
    campaign_service: domain::services::CampaignService,
    client_service: domain::services::ClientService,
    campaign_stat_service: domain::services::CampaignStatService,
    redis_service: domain::services::RedisService<'p>,
    db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
}

impl<'p> AdsClickUsecase<'p> {
    pub fn new(
        db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
        redis_pool: &'p infrastructure::database_connection::redis::RedisPool,
    ) -> Self {
        Self {
            campaign_service: domain::services::CampaignService,
            campaign_stat_service: domain::services::CampaignStatService,
            client_service: domain::services::ClientService,
            redis_service: domain::services::RedisService::new(redis_pool),
            db_pool,
        }
    }

    pub async fn click(
        &self,
        campaign_id: uuid::Uuid,
        click_request: domain::schemas::AdClickRequest,
    ) -> domain::services::ServiceResult<()> {
        let client = self
            .client_service
            .get_by_id(
                click_request.client_id,
                infrastructure::repository::sqlx_lib::PgClientRepository::new(self.db_pool),
            )
            .await?;

        if !self
            .campaign_service
            .campaign_is_exist(
                campaign_id,
                infrastructure::repository::sqlx_lib::PgCampaignRepository::new(self.db_pool),
            )
            .await?
        {
            return Err(domain::services::ServiceError::Validation(
                "campaign_id does not exist".into(),
            ));
        }
        let advanced_time = self.redis_service.get_advance_time().await?;
        let mut campaign = self.redis_service.get_active_campaign(&campaign_id).await?;

        if !campaign.view_clients_id.contains(&client.client_id) {
            return Err(domain::services::ServiceError::Validation(
                "client never view this this campaign".into(),
            ));
        }

        self.campaign_stat_service
            .click_campaign(
                campaign_id,
                client.client_id,
                campaign.cost_per_click,
                advanced_time,
                infrastructure::repository::sqlx_lib::PgCampaignRepository::new(self.db_pool),
            )
            .await?;

        if campaign.click_clients_id.contains(&client.client_id) {
            return Ok(());
        }

        campaign.click_clients_id.push(client.client_id);

        domain::services::PrometheusService::ads_clicks(advanced_time, campaign.cost_per_click);

        self.redis_service.set_active_campaign(campaign).await?;

        Ok(())
    }
}
