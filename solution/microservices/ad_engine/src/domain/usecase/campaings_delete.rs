use crate::{domain, infrastructure};

pub struct CampaignsDeleteUsecase<'p> {
    campaign_service: domain::services::CampaignService<'p>,
}

impl<'p> CampaignsDeleteUsecase<'p> {
    pub fn new(db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool) -> Self {
        Self {
            campaign_service: domain::services::CampaignService::new(db_pool),
        }
    }

    pub async fn delete(
        self,
        advertiser_id: uuid::Uuid,
        campaign_id: uuid::Uuid,
    ) -> domain::services::ServiceResult<()> {
        self.campaign_service
            .delete::<infrastructure::repository::sqlx_lib::PgCampaignRepository>(advertiser_id, campaign_id)
            .await
    }
}
