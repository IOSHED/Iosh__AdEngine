use async_trait::async_trait;

use crate::{domain, infrastructure};

/// Trait for retrieving campaign image names from a repository
///
/// This trait defines the contract for accessing campaign image names stored in
/// a repository. Implementations should handle the underlying storage and
/// retrieval mechanisms.
#[async_trait]
pub trait IGetCampaignNamesImage {
    /// Retrieves all image names associated with a campaign
    ///
    /// # Arguments
    /// * `campaign_id` - The UUID of the campaign to get image names for
    ///
    /// # Returns
    /// A `RepoResult` containing a vector of image names as strings if
    /// successful
    async fn get_names(&self, campaign_id: uuid::Uuid) -> infrastructure::repository::RepoResult<Vec<String>>;
}

/// Trait for retrieving campaign images from a repository
///
/// This trait defines the contract for accessing campaign image data stored in
/// a repository. Implementations should handle the underlying storage and
/// retrieval mechanisms.
#[async_trait]
pub trait IGetCampaignImage {
    /// Retrieves a specific campaign image and its metadata
    ///
    /// # Arguments
    /// * `campaign_id` - The UUID of the campaign the image belongs to
    /// * `advertiser_id` - The UUID of the advertiser who owns the campaign
    /// * `file_name` - The name of the image file to retrieve
    ///
    /// # Returns
    /// A `RepoResult` containing a tuple of (filename, image data) if
    /// successful
    async fn get(
        &self,
        campaign_id: uuid::Uuid,
        advertiser_id: uuid::Uuid,
        file_name: String,
    ) -> infrastructure::repository::RepoResult<(String, Vec<u8>)>;
}

/// Trait for deleting campaign images from a repository
///
/// This trait defines the contract for removing campaign images from a
/// repository. Implementations should handle the underlying deletion
/// mechanisms.
#[async_trait]
pub trait IDeleteCampaignImage {
    /// Deletes a specific campaign image
    ///
    /// # Arguments
    /// * `campaign_id` - The UUID of the campaign the image belongs to
    /// * `advertiser_id` - The UUID of the advertiser who owns the campaign
    /// * `file_name` - The name of the image file to delete
    ///
    /// # Returns
    /// A `RepoResult` containing unit type if successful
    async fn delete(
        &self,
        campaign_id: uuid::Uuid,
        advertiser_id: uuid::Uuid,
        file_name: String,
    ) -> infrastructure::repository::RepoResult<()>;
}

/// Service for managing campaign images
///
/// This service provides high-level operations for working with campaign
/// images, abstracting away the underlying repository implementation details.
#[derive(std::fmt::Debug)]
pub struct CampaignImageService;

impl<'p> CampaignImageService {
    /// Retrieves all image names for a campaign
    ///
    /// # Arguments
    /// * `campaign_id` - The UUID of the campaign to get image names for
    /// * `repo` - The repository implementation to use
    ///
    /// # Returns
    /// A `ServiceResult` containing a vector of image names if successful
    pub async fn get_names<R: IGetCampaignNamesImage>(
        &self,
        campaign_id: uuid::Uuid,
        repo: R,
    ) -> domain::services::ServiceResult<Vec<String>> {
        repo.get_names(campaign_id)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))
    }

    /// Retrieves a specific campaign image and its metadata
    ///
    /// # Arguments
    /// * `campaign_id` - The UUID of the campaign the image belongs to
    /// * `advertiser_id` - The UUID of the advertiser who owns the campaign
    /// * `file_name` - The name of the image file to retrieve
    /// * `repo` - The repository implementation to use
    ///
    /// # Returns
    /// A `ServiceResult` containing a tuple of (filename, image data) if
    /// successful
    pub async fn get<R: IGetCampaignImage>(
        &self,
        campaign_id: uuid::Uuid,
        advertiser_id: uuid::Uuid,
        file_name: String,
        repo: R,
    ) -> domain::services::ServiceResult<(String, Vec<u8>)> {
        repo.get(campaign_id, advertiser_id, file_name)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))
    }

    /// Deletes a specific campaign image
    ///
    /// # Arguments
    /// * `campaign_id` - The UUID of the campaign the image belongs to
    /// * `advertiser_id` - The UUID of the advertiser who owns the campaign
    /// * `file_name` - The name of the image file to delete
    /// * `repo` - The repository implementation to use
    ///
    /// # Returns
    /// A `ServiceResult` containing unit type if successful
    pub async fn delete<R: IDeleteCampaignImage>(
        &self,
        campaign_id: uuid::Uuid,
        advertiser_id: uuid::Uuid,
        file_name: String,
        repo: R,
    ) -> domain::services::ServiceResult<()> {
        repo.delete(campaign_id, advertiser_id, file_name)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;
    struct MockCampaignNamesImageRepo {
        names: Vec<String>,
    }

    #[async_trait]
    impl IGetCampaignNamesImage for MockCampaignNamesImageRepo {
        async fn get_names(&self, _campaign_id: Uuid) -> infrastructure::repository::RepoResult<Vec<String>> {
            Ok(self.names.clone())
        }
    }

    struct MockCampaignImageRepo {
        image_data: Option<(String, Vec<u8>)>,
    }

    #[async_trait]
    impl IGetCampaignImage for MockCampaignImageRepo {
        async fn get(
            &self,
            _campaign_id: Uuid,
            _advertiser_id: Uuid,
            _file_name: String,
        ) -> infrastructure::repository::RepoResult<(String, Vec<u8>)> {
            if let Some(data) = &self.image_data {
                Ok(data.clone())
            } else {
                Err(infrastructure::repository::RepoError::ObjDoesNotExists("obj".into()))
            }
        }
    }

    #[tokio::test]
    async fn test_get_names_success() {
        let campaign_id = Uuid::new_v4();
        let mock_repo = MockCampaignNamesImageRepo {
            names: vec!["image1.png".to_string(), "image2.png".to_string()],
        };
        let service = CampaignImageService;

        let result = service.get_names(campaign_id, mock_repo).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec!["image1.png", "image2.png"]);
    }

    #[tokio::test]
    async fn test_get_image_success() {
        let campaign_id = Uuid::new_v4();
        let advertiser_id = Uuid::new_v4();
        let file_name = "image1.png".to_string();

        let mock_repo = MockCampaignImageRepo {
            image_data: Some((file_name.clone(), vec![1, 2, 3, 4])),
        };
        let service = CampaignImageService;

        let result = service
            .get(campaign_id, advertiser_id, file_name.clone(), mock_repo)
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (file_name, vec![1, 2, 3, 4]));
    }

    #[tokio::test]
    async fn test_get_image_not_found() {
        let campaign_id = Uuid::new_v4();
        let advertiser_id = Uuid::new_v4();
        let file_name = "image_not_found.png".to_string();

        let mock_repo = MockCampaignImageRepo { image_data: None };
        let service = CampaignImageService;

        let result = service.get(campaign_id, advertiser_id, file_name, mock_repo).await;

        assert!(result.is_err());
    }
}
