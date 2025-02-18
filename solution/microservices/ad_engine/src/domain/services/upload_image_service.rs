use async_trait::async_trait;

use crate::{domain, infrastructure};

/// Trait defining interface for campaign image upload operations
#[async_trait]
pub trait IUploadCampaignImage {
    /// Uploads images for a campaign
    ///
    /// # Arguments
    /// * `campaign_id` - Unique identifier for the campaign
    /// * `media_max_image_on_campaign` - Maximum number of images allowed per
    ///   campaign
    /// * `files` - Vector of tuples containing (filename, file bytes, mime
    ///   type)
    ///
    /// # Returns
    /// * `RepoResult<()>` - Result indicating success or repository error
    async fn upload(
        &self,
        campaign_id: uuid::Uuid,
        media_max_image_on_campaign: usize,
        files: Vec<(String, Vec<u8>, String)>,
    ) -> infrastructure::repository::RepoResult<()>;
}

/// Service for handling campaign image uploads
#[derive(std::fmt::Debug)]
pub struct UploadImageService;

impl UploadImageService {
    /// Uploads images for a campaign using the provided repository
    ///
    /// # Arguments
    /// * `campaign_id` - Unique identifier for the campaign
    /// * `media_max_image_on_campaign` - Maximum number of images allowed
    /// * `files` - Vector of tuples containing (filename, file bytes, mime
    ///   type)
    /// * `repo` - Repository implementing IUploadCampaignImage trait
    ///
    /// # Returns
    /// * `ServiceResult<()>` - Result indicating success or service error
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
