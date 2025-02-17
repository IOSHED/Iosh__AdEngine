use async_trait::async_trait;

use crate::{domain, infrastructure};

#[async_trait]
pub trait IUploadCampaignImage {
    async fn upload(
        &self,
        campaign_id: uuid::Uuid,
        media_max_image_on_campaign: usize,
        files: Vec<(String, Vec<u8>, String)>,
    ) -> infrastructure::repository::RepoResult<()>;
}

#[derive(std::fmt::Debug)]
pub struct UploadImageService;

impl<'p> UploadImageService {
    pub async fn upload_for_campaign<R: IUploadCampaignImage>(
        &self,
        campaign_id: uuid::Uuid,
        media_max_image_on_campaign: usize,
        files: Vec<(String, Vec<u8>, String)>,
        repo: R,
    ) -> domain::services::ServiceResult<()> {
        repo.upload(campaign_id, media_max_image_on_campaign, files)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;
        Ok(())
    }
}
