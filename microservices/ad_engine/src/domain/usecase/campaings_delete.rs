use crate::{
    domain,
    infrastructure::{self, repository::IRepo},
};

pub struct CampaignsDeleteUsecase<'p> {
    campaign_service: domain::services::CampaignService,
    redis_service: domain::services::RedisService<'p>,
    db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
}

impl<'p> CampaignsDeleteUsecase<'p> {
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

    pub async fn delete(
        self,
        advertiser_id: uuid::Uuid,
        campaign_id: uuid::Uuid,
    ) -> domain::services::ServiceResult<()> {
        match self
            .campaign_service
            .delete(
                advertiser_id,
                campaign_id,
                infrastructure::repository::sqlx_lib::PgCampaignRepository::new(self.db_pool),
            )
            .await
        {
            Ok(_) => {
                self.redis_service.del_active_campaigns(&campaign_id).await?;
                let advanced_time = self.redis_service.get_advance_time().await?;
                domain::services::PrometheusService::increment_campaign_deleted(advanced_time);
                Ok(())
            },
            Err(e) => Err(e),
        }
    }
}
