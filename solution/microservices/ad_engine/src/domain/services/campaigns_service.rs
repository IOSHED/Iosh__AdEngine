use async_trait::async_trait;
use bigdecimal::ToPrimitive;

use crate::{domain, infrastructure};

/// Defines the interface for creating new campaigns in the system.
#[async_trait]
pub trait ICreateCampaign {
    /// Creates a new campaign for the given advertiser.
    ///
    /// # Arguments
    /// * `campaign` - The campaign creation request containing all required
    ///   fields
    /// * `advertiser_id` - Unique identifier of the advertiser creating the
    ///   campaign
    ///
    /// # Returns
    /// A Result containing the created campaign details or a repository error
    async fn create(
        &self,
        campaign: domain::schemas::CampaignsCreateRequest,
        advertiser_id: uuid::Uuid,
    ) -> infrastructure::repository::RepoResult<infrastructure::repository::sqlx_lib::CampaignReturningSchema>;
}

/// Defines the interface for updating existing campaigns.
#[async_trait]
pub trait IUpdateCampaign {
    /// Updates an existing campaign with new details.
    ///
    /// # Arguments
    /// * `campaign` - The campaign update request containing fields to modify
    /// * `advertiser_id` - ID of the advertiser who owns the campaign
    /// * `campaign_id` - Unique identifier of the campaign to update
    ///
    /// # Returns
    /// A Result containing the updated campaign details or a repository error
    async fn update(
        &self,
        campaign: domain::schemas::CampaignsUpdateRequest,
        advertiser_id: uuid::Uuid,
        campaign_id: uuid::Uuid,
    ) -> infrastructure::repository::RepoResult<infrastructure::repository::sqlx_lib::CampaignReturningSchema>;
}

/// Defines the interface for retrieving individual campaigns by ID.
#[async_trait]
pub trait IGetCampaignById {
    /// Retrieves a specific campaign by its ID and advertiser.
    ///
    /// # Arguments
    /// * `advertiser_id` - ID of the advertiser who owns the campaign
    /// * `campaign_id` - Unique identifier of the campaign to retrieve
    ///
    /// # Returns
    /// A Result containing the campaign details or a repository error
    async fn get_by_id(
        &self,
        advertiser_id: uuid::Uuid,
        campaign_id: uuid::Uuid,
    ) -> infrastructure::repository::RepoResult<infrastructure::repository::sqlx_lib::CampaignReturningSchema>;
}

/// Defines the interface for retrieving paginated lists of campaigns.
#[async_trait]
pub trait IGetCampaignList {
    /// Retrieves a paginated list of campaigns for an advertiser.
    ///
    /// # Arguments
    /// * `advertiser_id` - ID of the advertiser whose campaigns to retrieve
    /// * `size` - Number of items per page
    /// * `page` - Page number to retrieve
    ///
    /// # Returns
    /// A Result containing total count and vector of campaigns, or a repository
    /// error
    async fn get_list(
        &self,
        advertiser_id: uuid::Uuid,
        size: u32,
        page: u32,
    ) -> infrastructure::repository::RepoResult<(u64, Vec<infrastructure::repository::sqlx_lib::CampaignReturningSchema>)>;
}

/// Defines the interface for retrieving active campaigns.
#[async_trait]
pub trait IGetActiveCampaignList {
    /// Retrieves all currently active campaigns based on the given date.
    ///
    /// # Arguments
    /// * `current_date` - The date to check campaign activity against
    ///
    /// # Returns
    /// A Result containing a vector of active campaigns or a repository error
    async fn get_active_campaigns(
        &self,
        current_date: u32,
    ) -> infrastructure::repository::RepoResult<Vec<infrastructure::repository::sqlx_lib::CampaignReturningSchema>>;
}

/// Defines the interface for deleting campaigns.
#[async_trait]
pub trait IDeleteCampaign {
    /// Deletes a specific campaign.
    ///
    /// # Arguments
    /// * `advertiser_id` - ID of the advertiser who owns the campaign
    /// * `campaign_id` - Unique identifier of the campaign to delete
    ///
    /// # Returns
    /// A Result indicating success or a repository error
    async fn delete(
        &self,
        advertiser_id: uuid::Uuid,
        campaign_id: uuid::Uuid,
    ) -> infrastructure::repository::RepoResult<()>;
}

/// Defines the interface for retrieving campaign IDs.
#[async_trait]
pub trait IGetIdsCampaign {
    /// Retrieves all campaign IDs associated with an advertiser.
    ///
    /// # Arguments
    /// * `advertiser_id` - ID of the advertiser whose campaign IDs to retrieve
    ///
    /// # Returns
    /// A Result containing a vector of campaign UUIDs or a repository error
    async fn get_campaign_ids(
        &self,
        advertiser_id: uuid::Uuid,
    ) -> infrastructure::repository::RepoResult<Vec<uuid::Uuid>>;
}

/// Defines the interface for searching campaigns.
#[async_trait]
pub trait ISearchCampaign {
    /// Checks if a campaign exists by its ID.
    ///
    /// # Arguments
    /// * `campaign_id` - Unique identifier of the campaign to check
    ///
    /// # Returns
    /// A Result containing a boolean indicating existence or a repository error
    async fn are_exist(&self, campaign_id: uuid::Uuid) -> infrastructure::repository::RepoResult<bool>;
}

/// Service implementation for managing advertising campaigns.
#[derive(Debug)]
pub struct CampaignService;

impl<'p> CampaignService {
    /// Creates a new campaign with the provided details.
    ///
    /// # Arguments
    /// * `campaign` - Campaign creation request
    /// * `advertiser_id` - ID of the advertiser creating the campaign
    /// * `repo` - Repository implementation for campaign creation
    ///
    /// # Returns
    /// A ServiceResult containing the created campaign schema or an error
    #[tracing::instrument(name = "`CampaignService` create campaign", skip(repo))]
    pub async fn create<R: ICreateCampaign>(
        &self,
        campaign: domain::schemas::CampaignsCreateRequest,
        advertiser_id: uuid::Uuid,
        repo: R,
    ) -> domain::services::ServiceResult<domain::schemas::CampaignSchema> {
        let repo_campaign = repo
            .create(campaign, advertiser_id)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;

        Ok(repo_campaign.into())
    }

    /// Updates an existing campaign with new details, validating that certain
    /// fields cannot be modified after campaign start.
    ///
    /// # Arguments
    /// * `campaign` - The campaign update request containing fields to modify
    /// * `advertiser_id` - ID of the advertiser who owns the campaign
    /// * `campaign_id` - Unique identifier of the campaign to update
    /// * `time_advance` - Current timestamp to validate against campaign start
    ///   date
    /// * `repo` - Repository implementation for campaign operations
    ///
    /// # Returns
    /// A `ServiceResult` containing the updated campaign schema if successful,
    /// or a service error if:
    /// - The campaign does not exist
    /// - The advertiser does not own the campaign
    /// - Attempting to modify restricted fields after campaign start
    /// - Repository operations fail

    #[tracing::instrument(name = "`CampaignService` update campaign", skip(repo))]
    pub async fn update<R: IUpdateCampaign + IGetCampaignById>(
        &self,
        campaign: domain::schemas::CampaignsUpdateRequest,
        advertiser_id: uuid::Uuid,
        campaign_id: uuid::Uuid,
        time_advance: u32,
        repo: R,
    ) -> domain::services::ServiceResult<domain::schemas::CampaignSchema> {
        let old_campaign: domain::schemas::CampaignSchema = repo
            .get_by_id(advertiser_id, campaign_id)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?
            .into();

        if (time_advance >= old_campaign.start_date)
            & ((old_campaign.impressions_limit != campaign.impressions_limit)
                | (old_campaign.clicks_limit != campaign.clicks_limit)
                | (old_campaign.end_date != campaign.end_date)
                | (old_campaign.start_date != campaign.start_date))
        {
            return Err(domain::services::ServiceError::Validation(
                "Fields impressions_limit, clicks_limit, end_date, start_date don't update before start compaign"
                    .into(),
            ));
        }

        let repo_campaign = repo
            .update(campaign, advertiser_id, campaign_id)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;

        Ok(repo_campaign.into())
    }

    /// Deletes a campaign after verifying ownership.
    ///
    /// # Arguments
    /// * `advertiser_id` - ID of the advertiser attempting to delete the
    ///   campaign
    /// * `campaign_id` - Unique identifier of the campaign to delete
    /// * `repo` - Repository implementation for campaign operations
    ///
    /// # Returns
    /// A `ServiceResult` containing unit type if successful, or a service error
    /// if:
    /// - The campaign does not exist
    /// - The advertiser does not own the campaign
    /// - Repository operations fail

    #[tracing::instrument(name = "`CampaignService` delete campaign", skip(repo))]
    pub async fn delete<R: IDeleteCampaign + IGetCampaignById>(
        &self,
        advertiser_id: uuid::Uuid,
        campaign_id: uuid::Uuid,
        repo: R,
    ) -> domain::services::ServiceResult<()> {
        let campaign = repo
            .get_by_id(advertiser_id, campaign_id)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;

        repo.delete(campaign.advertiser_id, campaign.id)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;

        Ok(())
    }

    #[tracing::instrument(name = "`CampaignService` get campaign by id", skip(repo))]
    pub async fn get_by_id<R: IGetCampaignById>(
        &self,
        advertiser_id: uuid::Uuid,
        campaign_id: uuid::Uuid,
        repo: R,
    ) -> domain::services::ServiceResult<domain::schemas::CampaignSchema> {
        let repo_campaign = repo
            .get_by_id(advertiser_id, campaign_id)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;

        Ok(repo_campaign.into())
    }

    /// Retrieves a specific campaign by ID after verifying ownership.
    ///
    /// # Arguments
    /// * `advertiser_id` - ID of the advertiser who owns the campaign
    /// * `campaign_id` - Unique identifier of the campaign to retrieve
    /// * `repo` - Repository implementation for campaign operations
    ///
    /// # Returns
    /// A `ServiceResult` containing the campaign schema if found, or a service
    /// error if:
    /// - The campaign does not exist
    /// - The advertiser does not own the campaign
    /// - Repository operations fail
    #[tracing::instrument(name = "`CampaignService` get list of campaigns", skip(repo))]
    pub async fn get_list<R: IGetCampaignList>(
        &self,
        advertiser_id: uuid::Uuid,
        size: u32,
        page: u32,
        repo: R,
    ) -> domain::services::ServiceResult<(u64, Vec<domain::schemas::CampaignSchema>)> {
        let (total_count, campaigns) = repo
            .get_list(advertiser_id, size, page)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;

        let campaigns = campaigns.into_iter().map(|c| c.into()).collect();

        Ok((total_count, campaigns))
    }

    /// Retrieves all currently active campaigns based on the given date.
    ///
    /// # Arguments
    /// * `current_date` - The date to check campaign activity against
    /// * `repo` - Repository implementation for campaign operations
    ///
    /// # Returns
    /// A `ServiceResult` containing a vector of active campaign schemas,
    /// or a service error if repository operations fail

    #[tracing::instrument(name = "`CampaignService` get list of campaigns", skip(repo))]
    pub async fn get_active_campaigns<R: IGetActiveCampaignList>(
        &self,
        current_date: u32,
        repo: R,
    ) -> domain::services::ServiceResult<Vec<domain::schemas::CampaignSchema>> {
        let campaign = repo
            .get_active_campaigns(current_date)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;
        Ok(campaign.into_iter().map(|c| c.into()).collect())
    }

    /// Checks if a campaign exists by its ID.
    ///
    /// # Arguments
    /// * `campaign_id` - Unique identifier of the campaign to check
    /// * `repo` - Repository implementation for campaign operations
    ///
    /// # Returns
    /// A `ServiceResult` containing a boolean indicating if the campaign
    /// exists, or a service error if repository operations fail

    #[tracing::instrument(name = "`CampaignService` are exist campaign", skip(repo))]
    pub async fn campaign_is_exist<R: ISearchCampaign>(
        &self,
        campaign_id: uuid::Uuid,
        repo: R,
    ) -> domain::services::ServiceResult<bool> {
        repo.are_exist(campaign_id)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))
    }

    /// Retrieves all campaign IDs associated with an advertiser.
    ///
    /// # Arguments
    /// * `advertiser_id` - ID of the advertiser whose campaign IDs to retrieve
    /// * `repo` - Repository implementation for campaign operations
    ///
    /// # Returns
    /// A `ServiceResult` containing a vector of campaign UUIDs,
    /// or a service error if repository operations fail
    #[tracing::instrument(name = "`CampaignService` are exist campaign", skip(repo))]
    pub async fn get_campaign_ids<R: IGetIdsCampaign>(
        &self,
        advertiser_id: uuid::Uuid,
        repo: R,
    ) -> domain::services::ServiceResult<Vec<uuid::Uuid>> {
        repo.get_campaign_ids(advertiser_id)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))
    }
}

/// Implements conversion from repository campaign schema to domain campaign
/// schema.
impl From<infrastructure::repository::sqlx_lib::CampaignReturningSchema> for domain::schemas::CampaignSchema {
    /// Converts a repository campaign schema into a domain campaign schema.
    ///
    /// # Arguments
    /// * `campaign` - The repository campaign schema to convert
    ///
    /// # Returns
    /// A new domain campaign schema instance
    fn from(campaign: infrastructure::repository::sqlx_lib::CampaignReturningSchema) -> Self {
        Self {
            campaign_id: campaign.id,
            advertiser_id: campaign.advertiser_id,
            impressions_limit: campaign.impressions_limit as u32,
            clicks_limit: campaign.clicks_limit as u32,
            cost_per_impressions: campaign.cost_per_impressions.to_f64().unwrap_or(0.0),
            cost_per_clicks: campaign.cost_per_clicks.to_f64().unwrap_or(0.0),
            ad_title: campaign.ad_title,
            ad_text: campaign.ad_text,
            start_date: campaign.start_date as u32,
            end_date: campaign.end_date as u32,
            targeting: serde_json::from_value(campaign.targeting.unwrap_or(serde_json::json!({})))
                .unwrap_or(domain::schemas::TargetingCampaignSchema::default()),
        }
    }
}
