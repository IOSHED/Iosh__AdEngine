use async_trait::async_trait;

use crate::{domain, infrastructure};

#[async_trait]
pub trait IGetCampaignNamesImage {
    async fn get_names(&self, campaign_id: uuid::Uuid) -> infrastructure::repository::RepoResult<Vec<String>>;
}

#[async_trait]
pub trait IGetCampaignImage {
    async fn get(
        &self,
        campaign_id: uuid::Uuid,
        advertiser_id: uuid::Uuid,
        file_name: String,
    ) -> infrastructure::repository::RepoResult<(String, Vec<u8>)>;
}

#[async_trait]
pub trait IDeleteCampaignImage {
    async fn delete(
        &self,
        campaign_id: uuid::Uuid,
        advertiser_id: uuid::Uuid,
        file_name: String,
    ) -> infrastructure::repository::RepoResult<()>;
}

#[derive(std::fmt::Debug)]
pub struct CampaignImageService;

impl<'p> CampaignImageService {
    pub async fn get_names<R: IGetCampaignNamesImage>(
        &self,
        campaign_id: uuid::Uuid,
        repo: R,
    ) -> domain::services::ServiceResult<Vec<String>> {
        repo.get_names(campaign_id)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))
    }

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
