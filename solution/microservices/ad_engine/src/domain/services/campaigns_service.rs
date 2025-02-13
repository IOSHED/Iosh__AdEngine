use async_trait::async_trait;

use crate::{domain, infrastructure};

#[async_trait]
pub trait ICreateCampaign {
    async fn create(
        &self,
        campaign: domain::schemas::CampaignsCreateRequest,
        advertiser_id: uuid::Uuid,
        created_at: u32,
    ) -> infrastructure::repository::RepoResult<domain::schemas::CampaignSchema>;
}

#[async_trait]
pub trait IUpdateCampaign {
    async fn update(
        &self,
        campaign: domain::schemas::CampaignsUpdateRequest,
        advertiser_id: uuid::Uuid,
        campaign_id: uuid::Uuid,
        update_at: u32,
    ) -> infrastructure::repository::RepoResult<domain::schemas::CampaignSchema>;
}

#[async_trait]
pub trait IGetCampaignById {
    async fn get_by_id(
        &self,
        advertiser_id: uuid::Uuid,
        campaign_id: uuid::Uuid,
    ) -> infrastructure::repository::RepoResult<domain::schemas::CampaignSchema>;
}

#[async_trait]
pub trait IGetCampaignList {
    async fn get_list(
        &self,
        advertiser_id: uuid::Uuid,
        size: u32,
        page: u32,
    ) -> infrastructure::repository::RepoResult<(u64, Vec<domain::schemas::CampaignSchema>)>;
}

#[async_trait]
pub trait IDeleteCampaign {
    async fn delete(
        &self,
        advertiser_id: uuid::Uuid,
        campaign_id: uuid::Uuid,
    ) -> infrastructure::repository::RepoResult<()>;
}

#[derive(Debug)]
pub struct CampaignService<'p> {
    db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
}

impl<'p> CampaignService<'p> {
    pub fn new(db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool) -> Self {
        Self { db_pool }
    }
}

impl<'p> CampaignService<'p> {
    #[tracing::instrument(name = "`CampaignService` create campaign")]
    pub async fn create(
        self,
        campaign: domain::schemas::CampaignsCreateRequest,
        advertiser_id: uuid::Uuid,
        time_advance: u32,
    ) -> domain::services::ServiceResult<domain::schemas::CampaignSchema> {
        let repo_campaign = infrastructure::repository::sqlx_lib::PgCampaignRepository::new(self.db_pool)
            .create(campaign, advertiser_id, time_advance)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;

        Ok(repo_campaign)
    }

    #[tracing::instrument(name = "`CampaignService` update campaign")]
    pub async fn update(
        self,
        campaign: domain::schemas::CampaignsUpdateRequest,
        advertiser_id: uuid::Uuid,
        campaign_id: uuid::Uuid,
        time_advance: u32,
    ) -> domain::services::ServiceResult<domain::schemas::CampaignSchema> {
        let old_campaign = infrastructure::repository::sqlx_lib::PgCampaignRepository::new(self.db_pool)
            .get_by_id(advertiser_id, campaign_id)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;

        ((time_advance < campaign.start_date)
            & (old_campaign.impressions_limit == campaign.impressions_limit)
            & (old_campaign.clicks_limit == campaign.clicks_limit)
            & (old_campaign.end_date == campaign.end_date)
            & (old_campaign.start_date == campaign.start_date))
            .then_some(())
            .ok_or(domain::services::ServiceError::Validation(
                "Fields impressions_limit, clicks_limit, end_date, start_date don't update before start compaign"
                    .into(),
            ))?;

        let repo_campaign = infrastructure::repository::sqlx_lib::PgCampaignRepository::new(self.db_pool)
            .update(campaign, advertiser_id, campaign_id, time_advance)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;

        Ok(repo_campaign)
    }

    #[tracing::instrument(name = "`CampaignService` delete campaign")]
    pub async fn delete(
        self,
        advertiser_id: uuid::Uuid,
        campaign_id: uuid::Uuid,
    ) -> domain::services::ServiceResult<()> {
        infrastructure::repository::sqlx_lib::PgCampaignRepository::new(self.db_pool)
            .delete(advertiser_id, campaign_id)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;

        Ok(())
    }

    #[tracing::instrument(name = "`CampaignService` get campaign by id")]
    pub async fn get_by_id(
        self,
        advertiser_id: uuid::Uuid,
        campaign_id: uuid::Uuid,
    ) -> domain::services::ServiceResult<domain::schemas::CampaignSchema> {
        let repo_campaign = infrastructure::repository::sqlx_lib::PgCampaignRepository::new(self.db_pool)
            .get_by_id(advertiser_id, campaign_id)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;

        Ok(repo_campaign)
    }

    #[tracing::instrument(name = "`CampaignService` get list of campaigns")]
    pub async fn get_list(
        self,
        advertiser_id: uuid::Uuid,
        size: u32,
        page: u32,
    ) -> domain::services::ServiceResult<(u64, Vec<domain::schemas::CampaignSchema>)> {
        let (total_count, campaigns) = infrastructure::repository::sqlx_lib::PgCampaignRepository::new(self.db_pool)
            .get_list(advertiser_id, size, page)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;

        Ok((total_count, campaigns))
    }
}
