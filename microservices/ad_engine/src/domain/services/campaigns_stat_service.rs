use async_trait::async_trait;
use bigdecimal::ToPrimitive;

use crate::{domain, infrastructure};

/// Trait for getting or creating unique IDs for campaign statistics
///
/// This trait provides functionality to retrieve or generate unique identifiers
/// associated with campaign statistics tracking.
#[async_trait]
pub trait IGetOrCreateUniqIdForStatCampaign {
    /// Gets existing or creates new unique IDs for a campaign
    ///
    /// # Arguments
    /// * `campaign_id` - UUID of the campaign to get/create IDs for
    ///
    /// # Returns
    /// A tuple containing two vectors of UUIDs representing different ID sets
    async fn get_or_create_uniq_id(
        &self,
        campaign_id: uuid::Uuid,
    ) -> infrastructure::repository::RepoResult<(Vec<uuid::Uuid>, Vec<uuid::Uuid>)>;
}

/// Trait for recording campaign view events
///
/// Provides functionality to track when a campaign is viewed by a client.
#[async_trait]
pub trait IViewCampaign {
    /// Records a campaign view event
    ///
    /// # Arguments
    /// * `campaign_id` - UUID of the viewed campaign
    /// * `client_id` - UUID of the client viewing the campaign
    /// * `cost` - Cost associated with this view
    /// * `advanced_time` - Time spent viewing in seconds
    async fn view_campaign(
        &self,
        campaign_id: uuid::Uuid,
        client_id: uuid::Uuid,
        cost: f64,
        advanced_time: u32,
    ) -> infrastructure::repository::RepoResult<()>;
}

/// Trait for recording campaign click events
///
/// Provides functionality to track when a campaign is clicked by a client.
#[async_trait]
pub trait IClickCampaign {
    /// Records a campaign click event
    ///
    /// # Arguments
    /// * `campaign_id` - UUID of the clicked campaign
    /// * `client_id` - UUID of the client clicking the campaign
    /// * `cost` - Cost associated with this click
    /// * `advanced_time` - Time of interaction in seconds
    async fn click_campaign(
        &self,
        campaign_id: uuid::Uuid,
        client_id: uuid::Uuid,
        cost: f64,
        advanced_time: u32,
    ) -> infrastructure::repository::RepoResult<()>;
}

/// Trait for retrieving daily campaign statistics
///
/// Provides functionality to fetch aggregated daily statistics for campaigns.
#[async_trait]
pub trait IGetDailyStat {
    /// Retrieves daily statistics for a campaign
    ///
    /// # Arguments
    /// * `campaign_id` - UUID of the campaign to get statistics for
    async fn get_by_day(
        &self,
        campaign_id: uuid::Uuid,
    ) -> infrastructure::repository::RepoResult<Vec<infrastructure::repository::sqlx_lib::StatDailyReturningSchema>>;
}

/// Service for managing campaign statistics
///
/// Provides high-level business logic for tracking and analyzing campaign
/// performance metrics.
#[derive(std::fmt::Debug)]
pub struct CampaignStatService;

impl<'p> CampaignStatService {
    /// Gets or creates unique identifiers for campaign statistics tracking
    ///
    /// # Arguments
    /// * `campaign_id` - UUID of the target campaign
    /// * `repo` - Repository implementation for data access
    ///
    /// # Returns
    /// A tuple of UUID vectors representing different ID sets for the campaign
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

    /// Records a campaign view event
    ///
    /// # Arguments
    /// * `campaign_id` - UUID of the viewed campaign
    /// * `client_id` - UUID of the viewing client
    /// * `cost` - Associated view cost
    /// * `advanced_time` - View duration in seconds
    /// * `repo` - Repository implementation for data access
    #[tracing::instrument(name = "`CampaignStatService` add view to campaign", skip(repo))]
    pub async fn view_campaign<R: IViewCampaign>(
        &self,
        campaign_id: uuid::Uuid,
        client_id: uuid::Uuid,
        cost: f64,
        advanced_time: u32,
        repo: R,
    ) -> domain::services::ServiceResult<()> {
        repo.view_campaign(campaign_id, client_id, cost, advanced_time)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))
    }

    /// Records a campaign click event
    ///
    /// # Arguments
    /// * `campaign_id` - UUID of the clicked campaign
    /// * `client_id` - UUID of the clicking client
    /// * `cost` - Associated click cost
    /// * `advanced_time` - Interaction time in seconds
    /// * `repo` - Repository implementation for data access
    #[tracing::instrument(name = "`CampaignStatService` add click to campaign", skip(repo))]
    pub async fn click_campaign<R: IClickCampaign>(
        &self,
        campaign_id: uuid::Uuid,
        client_id: uuid::Uuid,
        cost: f64,
        advanced_time: u32,
        repo: R,
    ) -> domain::services::ServiceResult<()> {
        repo.click_campaign(campaign_id, client_id, cost, advanced_time)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))
    }

    /// Retrieves daily statistics for a campaign with gap filling
    ///
    /// Gets daily stats and fills any missing dates with default values to
    /// ensure continuous data series.
    ///
    /// # Arguments
    /// * `campaign_id` - UUID of the target campaign
    /// * `repo` - Repository implementation for data access
    ///
    /// # Returns
    /// Vector of daily statistics with gaps filled with default values
    #[tracing::instrument(name = "`CampaignStatService` add click to campaign", skip(repo))]
    pub async fn get_by_day<R: IGetDailyStat>(
        &self,
        campaign_id: uuid::Uuid,
        repo: R,
    ) -> domain::services::ServiceResult<Vec<domain::schemas::StatDailyResponse>> {
        let stats: Vec<domain::schemas::StatDailyResponse> = repo
            .get_by_day(campaign_id)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?
            .into_iter()
            .map(|s| s.into())
            .collect();

        if stats.is_empty() {
            return Ok(stats);
        }

        let start_date = stats.first().unwrap().date;
        let end_date = stats.last().unwrap().date;
        let mut filled_stats: Vec<domain::schemas::StatDailyResponse> = Vec::new();

        for date in start_date..=end_date {
            if let Some(stat) = stats.iter().find(|s| s.date == date) {
                filled_stats.push(stat.clone());
            } else {
                let mut default_stat = domain::schemas::StatDailyResponse::default();
                default_stat.date = date;
                filled_stats.push(default_stat);
            }
        }

        Ok(filled_stats)
    }
}

/// Conversion implementation for transforming database schema to domain
/// response
impl From<infrastructure::repository::sqlx_lib::StatDailyReturningSchema> for domain::schemas::StatDailyResponse {
    /// Converts database daily statistics to domain response format
    ///
    /// Handles null values and calculates derived metrics like conversion rate
    fn from(daily_stat: infrastructure::repository::sqlx_lib::StatDailyReturningSchema) -> Self {
        let impressions_count = daily_stat.impressions_count.unwrap_or(0);
        let clicks_count = daily_stat.clicks_count.unwrap_or(0);
        let spent_impressions = daily_stat
            .spent_impressions
            .as_ref()
            .and_then(|v| v.to_f64())
            .unwrap_or(0.0);
        let spent_clicks = daily_stat.spent_clicks.as_ref().and_then(|v| v.to_f64()).unwrap_or(0.0);
        let date = daily_stat.date.unwrap_or(0);

        let conversion = if impressions_count > 0 {
            (clicks_count as f64 / impressions_count as f64) * 100.0
        } else {
            0.
        };

        domain::schemas::StatDailyResponse {
            impressions_count: impressions_count as u32,
            clicks_count: clicks_count as u32,
            conversion,
            spent_impressions,
            spent_clicks,
            spent_total: spent_impressions + spent_clicks,
            date: date as u32,
        }
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use uuid::Uuid;

    use super::*;

    struct MockGetOrCreateUniqIdRepo {
        result: Result<(Vec<Uuid>, Vec<Uuid>), infrastructure::repository::RepoError>,
    }

    #[async_trait]
    impl IGetOrCreateUniqIdForStatCampaign for MockGetOrCreateUniqIdRepo {
        async fn get_or_create_uniq_id(
            &self,
            _campaign_id: Uuid,
        ) -> infrastructure::repository::RepoResult<(Vec<Uuid>, Vec<Uuid>)> {
            self.result.clone()
        }
    }

    struct MockViewCampaignRepo {
        result: Result<(), infrastructure::repository::RepoError>,
    }

    #[async_trait]
    impl IViewCampaign for MockViewCampaignRepo {
        async fn view_campaign(
            &self,
            _campaign_id: Uuid,
            _client_id: Uuid,
            _cost: f64,
            _advanced_time: u32,
        ) -> infrastructure::repository::RepoResult<()> {
            self.result.clone()
        }
    }

    struct MockClickCampaignRepo {
        result: Result<(), infrastructure::repository::RepoError>,
    }

    #[async_trait]
    impl IClickCampaign for MockClickCampaignRepo {
        async fn click_campaign(
            &self,
            _campaign_id: Uuid,
            _client_id: Uuid,
            _cost: f64,
            _advanced_time: u32,
        ) -> infrastructure::repository::RepoResult<()> {
            self.result.clone()
        }
    }

    struct MockGetDailyStatRepo {
        result: Result<
            Vec<infrastructure::repository::sqlx_lib::StatDailyReturningSchema>,
            infrastructure::repository::RepoError,
        >,
    }

    #[async_trait]
    impl IGetDailyStat for MockGetDailyStatRepo {
        async fn get_by_day(
            &self,
            _campaign_id: Uuid,
        ) -> infrastructure::repository::RepoResult<Vec<infrastructure::repository::sqlx_lib::StatDailyReturningSchema>>
        {
            self.result.clone()
        }
    }

    #[tokio::test]
    async fn test_get_or_create_uniq_id_success() {
        let campaign_id = Uuid::new_v4();
        let mock_repo = MockGetOrCreateUniqIdRepo {
            result: Ok((vec![Uuid::new_v4()], vec![Uuid::new_v4()])),
        };
        let service = CampaignStatService;

        let result = service.get_or_create_uniq_id(campaign_id, mock_repo).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().0.len(), 1);
    }

    #[tokio::test]
    async fn test_view_campaign_success() {
        let campaign_id = Uuid::new_v4();
        let client_id = Uuid::new_v4();
        let cost = 100.0;
        let advanced_time = 123;
        let mock_repo = MockViewCampaignRepo { result: Ok(()) };
        let service = CampaignStatService;

        let result = service
            .view_campaign(campaign_id, client_id, cost, advanced_time, mock_repo)
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_click_campaign_success() {
        let campaign_id = Uuid::new_v4();
        let client_id = Uuid::new_v4();
        let cost = 100.0;
        let advanced_time = 123;
        let mock_repo = MockClickCampaignRepo { result: Ok(()) };
        let service = CampaignStatService;
        let result = service
            .click_campaign(campaign_id, client_id, cost, advanced_time, mock_repo)
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_by_day_success() {
        let campaign_id = Uuid::new_v4();
        let mock_repo = MockGetDailyStatRepo {
            result: Ok(vec![infrastructure::repository::sqlx_lib::StatDailyReturningSchema {
                date: Some(1),
                impressions_count: None,
                clicks_count: None,
                spent_impressions: None,
                spent_clicks: None,
            }]),
        };
        let service = CampaignStatService;

        let result = service.get_by_day(campaign_id, mock_repo).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_by_day_empty() {
        let campaign_id = Uuid::new_v4();
        let mock_repo = MockGetDailyStatRepo { result: Ok(vec![]) };
        let service = CampaignStatService;

        let result = service.get_by_day(campaign_id, mock_repo).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }
}
