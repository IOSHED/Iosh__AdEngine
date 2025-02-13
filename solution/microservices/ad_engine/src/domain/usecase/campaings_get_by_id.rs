use crate::{domain, infrastructure};

pub struct CampaignsGetByIdUsecase<'p> {
    campaign_service: domain::services::CampaignService<'p>,
}

impl<'p> CampaignsGetByIdUsecase<'p> {
    pub fn new(db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool) -> Self {
        Self {
            campaign_service: domain::services::CampaignService::new(db_pool),
        }
    }

    pub async fn get(
        self,
        advertiser_id: uuid::Uuid,
        campaign_id: uuid::Uuid,
    ) -> domain::services::ServiceResult<domain::schemas::CampaignSchema> {
        self.campaign_service
            .get_by_id::<infrastructure::repository::sqlx_lib::PgCampaignRepository>(advertiser_id, campaign_id)
            .await
    }
}
