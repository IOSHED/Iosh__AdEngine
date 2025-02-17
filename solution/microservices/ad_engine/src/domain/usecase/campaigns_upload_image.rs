use crate::{
    domain,
    infrastructure::{self, repository::IRepo},
};

pub struct CampaignsUploadImageUsecase<'p> {
    upload_image_service: domain::services::UploadImageService,
    campaign_service: domain::services::CampaignService,
    db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
    media_max_image_on_campaign: usize,
}

impl<'p> CampaignsUploadImageUsecase<'p> {
    pub fn new(
        db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
        media_max_image_on_campaign: usize,
    ) -> Self {
        Self {
            upload_image_service: domain::services::UploadImageService,
            campaign_service: domain::services::CampaignService,
            db_pool,
            media_max_image_on_campaign,
        }
    }

    pub async fn upload(
        self,
        advertiser_id: uuid::Uuid,
        campaign_id: uuid::Uuid,
        files_buffer: Vec<(String, Vec<u8>, String)>,
    ) -> domain::services::ServiceResult<()> {
        if files_buffer.len() > self.media_max_image_on_campaign {
            return Err(domain::services::ServiceError::Validation(format!(
                "too many files, no more than {} on campaign",
                self.media_max_image_on_campaign
            )));
        }

        let campaign = self
            .campaign_service
            .get_by_id(
                advertiser_id,
                campaign_id,
                infrastructure::repository::sqlx_lib::PgCampaignRepository::new(self.db_pool),
            )
            .await?;

        self.upload_image_service
            .upload_for_campaign(
                campaign.campaign_id,
                self.media_max_image_on_campaign,
                files_buffer,
                infrastructure::repository::sqlx_lib::PgCampaignImageRepository::new(self.db_pool),
            )
            .await?;

        Ok(())
    }
}
