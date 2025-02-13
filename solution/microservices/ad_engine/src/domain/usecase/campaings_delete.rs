use crate::{
    domain,
    infrastructure::{self, repository::IRepo},
};

pub struct CampaignsDeleteUsecase<'p> {
    campaign_service: domain::services::CampaignService,
    db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
}

impl<'p> CampaignsDeleteUsecase<'p> {
    pub fn new(db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool) -> Self {
        Self {
            campaign_service: domain::services::CampaignService,
            db_pool,
        }
    }

    pub async fn delete(
        self,
        advertiser_id: uuid::Uuid,
        campaign_id: uuid::Uuid,
    ) -> domain::services::ServiceResult<()> {
        self.campaign_service
            .delete(
                advertiser_id,
                campaign_id,
                infrastructure::repository::sqlx_lib::PgCampaignRepository::new(self.db_pool),
            )
            .await
    }
}
