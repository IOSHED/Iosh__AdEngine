use crate::{
    domain,
    infrastructure::{self, repository::IRepo},
};

pub struct CampaignsGetNameImagesUsecase<'p> {
    campaign_image_service: domain::services::CampaignImageService,
    campaign_service: domain::services::CampaignService,
    db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
}

impl<'p> CampaignsGetNameImagesUsecase<'p> {
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
    ) -> domain::services::ServiceResult<Vec<String>> {
        let campaign = self
            .campaign_service
            .get_by_id(
                advertiser_id,
                campaign_id,
                infrastructure::repository::sqlx_lib::PgCampaignRepository::new(self.db_pool),
            )
            .await?;

        self.campaign_image_service
            .get_names(
                campaign.campaign_id,
                infrastructure::repository::sqlx_lib::PgCampaignImageRepository::new(self.db_pool),
            )
            .await
    }
}
