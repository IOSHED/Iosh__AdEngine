use async_trait::async_trait;

use crate::{domain, infrastructure};

#[async_trait]
pub trait IGetOrCreateUniqIdForStatCampaign {
    async fn get_or_create_uniq_id(
        &self,
        campaign_id: uuid::Uuid,
    ) -> infrastructure::repository::RepoResult<(Vec<uuid::Uuid>, Vec<uuid::Uuid>)>;
}

#[async_trait]
pub trait IViewCampaign {
    async fn view_campaign(
        &self,
        campaign_id: uuid::Uuid,
        client_id: uuid::Uuid,
    ) -> infrastructure::repository::RepoResult<()>;
}

#[derive(std::fmt::Debug)]
pub struct CampaignStatService;

impl<'p> CampaignStatService {
    #[tracing::instrument(name = "`CampaignStatService` get ot create uniq id for stats campaign", skip(repo))]
    pub async fn get_or_create_uniq_id<R: IGetOrCreateUniqIdForStatCampaign>(
        &self,
        campaign_id: uuid::Uuid,
        repo: R,
    ) -> domain::services::ServiceResult<(Vec<uuid::Uuid>, Vec<uuid::Uuid>)> {
        repo.get_or_create_uniq_id(campaign_id)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))
    }

    #[tracing::instrument(name = "`CampaignStatService` add view to campaign", skip(repo))]
    pub async fn view_campaign<R: IViewCampaign>(
        &self,
        campaign_id: uuid::Uuid,
        client_id: uuid::Uuid,
        repo: R,
    ) -> domain::services::ServiceResult<()> {
        repo.view_campaign(campaign_id, client_id)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))
    }
}
