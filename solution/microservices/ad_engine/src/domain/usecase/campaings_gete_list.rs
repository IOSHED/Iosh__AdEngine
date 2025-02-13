use crate::{domain, infrastructure};

pub struct CampaignsGetListUsecase<'p> {
    campaign_service: domain::services::CampaignService<'p>,
}

impl<'p> CampaignsGetListUsecase<'p> {
    pub fn new(db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool) -> Self {
        Self {
            campaign_service: domain::services::CampaignService::new(db_pool),
        }
    }

    pub async fn get(
        self,
        advertiser_id: uuid::Uuid,
        size: u32,
        page: u32,
    ) -> domain::services::ServiceResult<(u64, Vec<domain::schemas::CampaignSchema>)> {
        self.campaign_service
            .get_list::<infrastructure::repository::sqlx_lib::PgCampaignRepository>(advertiser_id, size, page)
            .await
    }
}
