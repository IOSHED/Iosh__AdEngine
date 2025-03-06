use crate::{
    domain,
    infrastructure::{self, repository::IRepo},
};

pub struct CampaignsGetListUsecase<'p> {
    campaign_service: domain::services::CampaignService,
    db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
}

impl<'p> CampaignsGetListUsecase<'p> {
    pub fn new(db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool) -> Self {
        Self {
            campaign_service: domain::services::CampaignService,
            db_pool,
        }
    }

    pub async fn get(
        self,
        advertiser_id: uuid::Uuid,
        size: u32,
        page: u32,
    ) -> domain::services::ServiceResult<(u64, Vec<domain::schemas::CampaignSchema>)> {
        self.campaign_service
            .get_list(
                advertiser_id,
                size,
                page,
                infrastructure::repository::sqlx_lib::PgCampaignRepository::new(self.db_pool),
            )
            .await
    }
}
