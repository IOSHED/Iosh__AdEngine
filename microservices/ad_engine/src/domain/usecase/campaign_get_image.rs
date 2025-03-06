use crate::{
    domain,
    infrastructure::{self, repository::IRepo},
};

pub struct CampaignsGetImageUsecase<'p> {
    campaign_image_service: domain::services::CampaignImageService,
    campaign_service: domain::services::CampaignService,
    db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
}

impl<'p> CampaignsGetImageUsecase<'p> {
    pub fn new(db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool) -> Self {
        Self {
            campaign_image_service: domain::services::CampaignImageService,
            campaign_service: domain::services::CampaignService,
            db_pool,
        }
    }

    pub async fn get(
        self,
        advertiser_id: uuid::Uuid,
        campaign_id: uuid::Uuid,
        file_name: String,
    ) -> domain::services::ServiceResult<(String, Vec<u8>)> {
        let campaign = self
            .campaign_service
            .get_by_id(
                advertiser_id,
                campaign_id,
                infrastructure::repository::sqlx_lib::PgCampaignRepository::new(self.db_pool),
            )
            .await?;

        self.campaign_image_service
            .get(
                campaign.campaign_id,
                campaign.advertiser_id,
                file_name,
                infrastructure::repository::sqlx_lib::PgCampaignImageRepository::new(self.db_pool),
            )
            .await
    }
}
