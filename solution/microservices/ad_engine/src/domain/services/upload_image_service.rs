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

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use mockall::{mock, predicate::*};

    use super::*;

    mock! {
        pub UploadCampaignImageMock {}

        #[async_trait]
        impl IUploadCampaignImage for UploadCampaignImageMock {
            async fn upload(
                &self,
                campaign_id: uuid::Uuid,
                media_max_image_on_campaign: usize,
                files: Vec<(String, Vec<u8>, String)>
            ) -> infrastructure::repository::RepoResult<()>;
        }
    }

    #[tokio::test]
    async fn test_upload_for_campaign_success() {
        let mut mock_repo = MockUploadCampaignImageMock::new();
        let service = UploadImageService;

        let campaign_id = uuid::Uuid::new_v4();
        let files = vec![
            ("image1.png".to_string(), vec![1, 2, 3], "image/png".to_string()),
            ("image2.jpg".to_string(), vec![4, 5, 6], "image/jpeg".to_string()),
        ];

        mock_repo
            .expect_upload()
            .with(eq(campaign_id), eq(2), eq(files.clone()))
            .times(1)
            .returning(|_, _, _| Ok(()));

        let result = service.upload_for_campaign(campaign_id, 2, files, mock_repo).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_upload_for_campaign_repository_error() {
        let mut mock_repo = MockUploadCampaignImageMock::new();
        let service = UploadImageService;

        let campaign_id = uuid::Uuid::new_v4();
        let files = vec![("image1.png".to_string(), vec![1, 2, 3], "image/png".to_string())];

        mock_repo
            .expect_upload()
            .with(eq(campaign_id), eq(1), eq(files.clone()))
            .times(1)
            .returning(|_, _, _| Err(infrastructure::repository::RepoError::Unknown));

        let result = service.upload_for_campaign(campaign_id, 1, files, mock_repo).await;

        assert!(result.is_err());
    }
}
