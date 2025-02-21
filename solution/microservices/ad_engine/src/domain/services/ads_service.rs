use async_trait::async_trait;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{domain, infrastructure};

/// Trait for retrieving machine learning scores for advertisers.
/// This trait is used to get relevance scores between a client and advertisers.
#[async_trait]
pub trait IGetMlScores {
    /// Gets ML scores for a client-advertiser pair
    ///
    /// # Arguments
    /// * `client_id` - UUID of the client
    /// * `advertisers_id` - Vector of advertiser UUIDs to get scores for
    ///
    /// # Returns
    /// * `RepoResult<Vec<f64>>` - Vector of scores between 0 and 1 for each
    ///   advertiser
    async fn get_ml_scores(
        &self,
        client_id: uuid::Uuid,
        advertisers_id: Vec<uuid::Uuid>,
    ) -> infrastructure::repository::RepoResult<Vec<f64>>;
}

/// Service for managing and scoring advertising campaigns
///
/// This service handles the core business logic for selecting and scoring ad
/// campaigns based on multiple weighted factors including profit potential,
/// relevance, fulfillment rate and time constraints.
#[derive(Debug)]
pub struct AdsService {
    weight_profit: f64,
    weight_relevance: f64,
    weight_fulfillment: f64,
    weight_time_left: f64,
}

impl AdsService {
    /// Creates a new AdsService instance with specified weights for scoring
    /// factors
    ///
    /// # Arguments
    /// * `weight_profit` - Weight factor for profit scoring (0-1)
    /// * `weight_relevance` - Weight factor for relevance scoring (0-1)
    /// * `weight_fulfillment` - Weight factor for campaign fulfillment (0-1)
    /// * `weight_time_left` - Weight factor for remaining campaign time (0-1)
    pub fn new(
        weight_profit: f64,
        weight_relevance: f64,
        weight_fulfillment: f64,
        weight_time_left: f64,
    ) -> Self {
        Self {
            weight_profit,
            weight_relevance,
            weight_fulfillment,
            weight_time_left,
        }
    }
}

impl AdsService {
    /// Recommends the most suitable ad for a client based on multiple factors
    ///
    /// # Arguments
    /// * `active_campaigns` - List of currently active ad campaigns
    /// * `client_id` - UUID of the target client
    /// * `advanced_time` - Current timestamp for time-based calculations
    /// * `repo_client` - Repository for accessing client data
    /// * `repo_score` - Repository for accessing ML scores
    ///
    /// # Returns
    /// * `ServiceResult<AdSchema>` - The recommended ad if found
    ///
    /// # Type Parameters
    /// * `R1` - Type implementing IGetClientById trait
    /// * `R2` - Type implementing IGetMlScores trait
    pub async fn recommendation_ads<R1, R2>(
        &self,
        active_campaigns: Vec<domain::schemas::ActiveCampaignSchema>,
        client_id: uuid::Uuid,
        advanced_time: u32,
        repo_client: R1,
        repo_score: R2,
    ) -> domain::services::ServiceResult<domain::schemas::AdSchema>
    where
        R1: super::repository::IGetClientById,
        R2: super::repository::IGetMlScores,
    {
        let client = self.get_client(repo_client, client_id).await?;
        let suitable_campaigns = self.get_suitable_campaigns(active_campaigns, &client).await?;
        let scored_campaigns = self
            .score_campaigns(suitable_campaigns, client_id, advanced_time, &repo_score)
            .await?;
        let top_campaign = self.get_top_campaign(&scored_campaigns).await?;

        Ok(domain::schemas::AdSchema {
            ad_id: top_campaign.campaign_id,
            ad_title: top_campaign.ad_title.clone(),
            ad_text: top_campaign.ad_text.clone(),
            advertiser_id: top_campaign.advertiser_id,
        })
    }

    /// Retrieves client information from the repository
    ///
    /// # Arguments
    /// * `repo_client` - Repository implementing IGetClientById
    /// * `client_id` - UUID of the client to retrieve
    async fn get_client<R>(
        &self,
        repo_client: R,
        client_id: uuid::Uuid,
    ) -> domain::services::ServiceResult<infrastructure::repository::sqlx_lib::ClientReturningSchema>
    where
        R: super::repository::IGetClientById,
    {
        repo_client
            .get_by_id(client_id)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))
    }

    /// Filters campaigns based on targeting criteria and campaign limits
    ///
    /// # Arguments
    /// * `active_campaigns` - List of active campaigns to filter
    /// * `client` - Client information for targeting matching
    ///
    /// # Returns
    /// * List of campaigns that match targeting criteria and have available
    ///   impressions
    async fn get_suitable_campaigns(
        &self,
        active_campaigns: Vec<domain::schemas::ActiveCampaignSchema>,
        client: &infrastructure::repository::sqlx_lib::ClientReturningSchema,
    ) -> domain::services::ServiceResult<Vec<domain::schemas::ActiveCampaignSchema>> {
        let filtered_campaigns = self
            .filter_targeted_campaigns(
                active_campaigns,
                client.age as u8,
                client.gender.clone(),
                client.location.clone(),
            )
            .await
            .into_iter()
            .filter(|c| {
                c.view_clients_id.len() <= (c.impressions_limit as f64 * 1.05).floor() as usize
                // && (
                //     (c.view_clients_id.contains(&client.client_id) && !c.click_clients_id.contains(&client.client_id)) 
                //     || !c.view_clients_id.contains(&client.client_id)
                // )
            })
            .collect::<Vec<_>>();

        if filtered_campaigns.is_empty() {
            return Err(domain::services::ServiceError::Repository(
                infrastructure::repository::RepoError::ObjDoesNotExists("Suitable campaigns".into()),
            ));
        }

        Ok(filtered_campaigns)
    }

    /// Scores campaigns based on multiple weighted factors
    ///
    /// # Arguments
    /// * `suitable_campaigns` - Pre-filtered list of suitable campaigns
    /// * `client_id` - Target client UUID
    /// * `advanced_time` - Current timestamp
    /// * `repo_score` - Repository for ML scores
    ///
    /// # Returns
    /// * Sorted vector of (score, end_date, campaign) tuples
    async fn score_campaigns<R>(
        &self,
        suitable_campaigns: Vec<domain::schemas::ActiveCampaignSchema>,
        client_id: uuid::Uuid,
        advanced_time: u32,
        repo_score: &R,
    ) -> domain::services::ServiceResult<Vec<(f64, u32, domain::schemas::ActiveCampaignSchema)>>
    where
        R: super::repository::IGetMlScores,
    {
        let profits: Vec<f64> = suitable_campaigns
            .par_iter()
            .map(|campaign| {
                let remaining_impressions = campaign.impressions_limit as f64 - campaign.view_clients_id.len() as f64;
                let remaining_clicks = campaign.clicks_limit as f64 - campaign.click_clients_id.len() as f64;
                (remaining_impressions * campaign.cost_per_impression as f64)
                    + (remaining_clicks * campaign.cost_per_click as f64)
            })
            .collect();

        let (min_profit, max_profit) = self.calculate_min_max(&profits);
        let advertisers_id: Vec<uuid::Uuid> = suitable_campaigns
            .iter()
            .map(|campaign| campaign.advertiser_id)
            .collect();
        let scores = repo_score
            .get_ml_scores(client_id, advertisers_id)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e.into()))?;
        let (min_score, max_score) = self.calculate_min_max(&scores);

        let mut scored_campaigns = self
            .calculate_scores(
                suitable_campaigns,
                &profits,
                &scores,
                min_profit,
                max_profit,
                min_score,
                max_score,
                advanced_time,
            )
            .await;
        scored_campaigns.sort_by(|(score_a, end_date_a, _), (score_b, end_date_b, _)| {
            score_b
                .partial_cmp(score_a)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(end_date_a.cmp(end_date_b))
        });

        Ok(scored_campaigns)
    }

    /// Gets the highest scoring campaign from scored campaigns list
    ///
    /// # Arguments
    /// * `scored_campaigns` - List of campaigns with their scores
    async fn get_top_campaign<'a>(
        &self,
        scored_campaigns: &'a [(f64, u32, domain::schemas::ActiveCampaignSchema)],
    ) -> domain::services::ServiceResult<&'a domain::schemas::ActiveCampaignSchema> {
        scored_campaigns
            .get(0)
            .map(|(_, _, campaign)| campaign)
            .ok_or_else(|| domain::services::ServiceError::Validation("No top campaign found".into()))
    }

    /// Filters campaigns based on targeting criteria
    ///
    /// # Arguments
    /// * `active_campaigns` - List of campaigns to filter
    /// * `age` - Client age for targeting
    /// * `gender` - Client gender for targeting
    /// * `location` - Client location for targeting
    async fn filter_targeted_campaigns(
        &self,
        active_campaigns: Vec<domain::schemas::ActiveCampaignSchema>,
        age: u8,
        gender: String,
        location: String,
    ) -> Vec<domain::schemas::ActiveCampaignSchema> {
        active_campaigns
            .into_iter()
            .filter(|c| {
                (c.targeting.location == Some(location.clone()) || c.targeting.location.is_none())
                    && (c.targeting.gender == Some(gender.clone())
                        || c.targeting.gender == Some("ALL".to_string())
                        || c.targeting.gender.is_none())
                    && (c.targeting.age_from <= Some(age) || c.targeting.age_from.is_none())
                    && (c.targeting.age_to >= Some(age) || c.targeting.age_to.is_none())
            })
            .collect()
    }

    /// Calculates minimum and maximum values from a slice of f64
    ///
    /// # Arguments
    /// * `values` - Slice of f64 values
    ///
    /// # Returns
    /// * Tuple of (minimum, maximum) values
    fn calculate_min_max(&self, values: &[f64]) -> (f64, f64) {
        let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        (min, max)
    }

    /// Calculates final scores for campaigns considering all weighted factors
    ///
    /// # Arguments
    /// * `campaigns` - List of campaigns to score
    /// * `profits` - Pre-calculated profit values
    /// * `scores` - ML relevance scores
    /// * `min_profit` - Minimum profit value for normalization
    /// * `max_profit` - Maximum profit value for normalization
    /// * `min_score` - Minimum ML score for normalization
    /// * `max_score` - Maximum ML score for normalization
    /// * `advanced_time` - Current timestamp
    async fn calculate_scores(
        &self,
        campaigns: Vec<domain::schemas::ActiveCampaignSchema>,
        profits: &[f64],
        scores: &[f64],
        min_profit: f64,
        max_profit: f64,
        min_score: f64,
        max_score: f64,
        advanced_time: u32,
    ) -> Vec<(f64, u32, domain::schemas::ActiveCampaignSchema)> {
        futures::future::join_all(campaigns.into_iter().enumerate().map(|(i, campaign)| {
            let profit = profits[i];
            let score = scores[i];
            async move {
                let normalized_profit = self.normalize_value(profit, min_profit, max_profit);
                let normalized_relevance = self.normalize_value(score, min_score, max_score);
                let fulfillment = self.calculate_fulfillment(&campaign);
                let normalized_time_left = self.calculate_time_left(campaign.end_date, advanced_time);
                let combined_score: f64 = self.weight_profit * normalized_profit
                    + self.weight_relevance * normalized_relevance
                    + self.weight_fulfillment * fulfillment
                    + self.weight_time_left * normalized_time_left;
                (combined_score, campaign.end_date, campaign)
            }
        }))
        .await
    }

    /// Normalizes a value using natural logarithm scaling
    ///
    /// # Arguments
    /// * `value` - Value to normalize
    /// * `min` - Minimum value in range
    /// * `max` - Maximum value in range
    fn normalize_value(&self, value: f64, min: f64, max: f64) -> f64 {
        if max != min {
            return ((value + 1.0).ln() - (min + 1.0).ln()) / ((max + 1.0).ln() - (min + 1.0).ln());
        }
        0.0
    }

    /// Calculates campaign fulfillment rate based on remaining impressions and
    /// clicks
    ///
    /// # Arguments
    /// * `campaign` - Campaign to calculate fulfillment for
    fn calculate_fulfillment(&self, campaign: &domain::schemas::ActiveCampaignSchema) -> f64 {
        let remaining_impressions = campaign.impressions_limit as f64 - campaign.view_clients_id.len() as f64;
        let remaining_clicks = campaign.clicks_limit as f64 - campaign.click_clients_id.len() as f64;
        (remaining_impressions / campaign.impressions_limit as f64) + (remaining_clicks / campaign.clicks_limit as f64)
    }

    /// Calculates normalized time remaining for a campaign
    ///
    /// # Arguments
    /// * `end_date` - Campaign end timestamp
    /// * `advanced_time` - Current timestamp
    fn calculate_time_left(&self, end_date: u32, advanced_time: u32) -> f64 {
        1.0 - ((end_date as f64 - advanced_time as f64) / u32::MAX as f64).clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use domain::services::ServiceError;
    use infrastructure::repository::RepoError;
    use mockall::{mock, predicate::*};
    use uuid::Uuid;

    use super::*;
    use crate::domain::services::repository::IGetClientById;

    mock! {
        pub ClientRepo {}
        #[async_trait]
        impl IGetClientById for ClientRepo {
            async fn get_by_id(&self, id: Uuid) -> infrastructure::repository::RepoResult<infrastructure::repository::sqlx_lib::ClientReturningSchema>;
        }
    }

    mock! {
        pub MlScoreRepo {}
        #[async_trait]
        impl IGetMlScores for MlScoreRepo {
            async fn get_ml_scores(&self, client_id: Uuid, advertisers_id: Vec<Uuid>) -> infrastructure::repository::RepoResult<Vec<f64>>;
        }
    }

    fn create_test_service() -> AdsService {
        AdsService::new(0.4, 0.3, 0.2, 0.1)
    }

    fn create_test_campaign(id: Uuid, advertiser_id: Uuid) -> domain::schemas::ActiveCampaignSchema {
        domain::schemas::ActiveCampaignSchema {
            campaign_id: id,
            advertiser_id,
            ad_title: "Test Ad".into(),
            ad_text: "Test Content".into(),
            cost_per_impression: 1.,
            cost_per_click: 2.,
            impressions_limit: 100,
            clicks_limit: 50,
            start_date: 0,
            end_date: 100,
            targeting: domain::schemas::TargetingCampaignSchema {
                age_from: Some(18),
                age_to: Some(35),
                gender: Some("Male".into()),
                location: Some("NY".into()),
            },
            view_clients_id: vec![],
            click_clients_id: vec![],
        }
    }

    #[tokio::test]
    async fn recommendation_ads_success() {
        let client_id = Uuid::new_v4();
        let campaign_id = Uuid::new_v4();
        let advertiser_id = Uuid::new_v4();

        let mut mock_client_repo = MockClientRepo::new();
        mock_client_repo.expect_get_by_id().returning(move |_| {
            Ok(infrastructure::repository::sqlx_lib::ClientReturningSchema {
                client_id,
                login: "my_name".into(),
                age: 25,
                gender: "Male".into(),
                location: "NY".into(),
            })
        });

        let mut mock_ml_repo = MockMlScoreRepo::new();
        mock_ml_repo.expect_get_ml_scores().returning(|_, _| Ok(vec![0.8]));

        let service = create_test_service();
        let result = service
            .recommendation_ads(
                vec![create_test_campaign(campaign_id, advertiser_id)],
                client_id,
                50,
                mock_client_repo,
                mock_ml_repo,
            )
            .await;

        assert!(result.is_ok());
        let ad = result.unwrap();
        assert_eq!(ad.ad_id, campaign_id);
    }

    #[tokio::test]
    async fn no_suitable_campaigns_error() {
        let client_id = Uuid::new_v4();

        let mut mock_client_repo = MockClientRepo::new();
        mock_client_repo.expect_get_by_id().returning(move |_| {
            Ok(infrastructure::repository::sqlx_lib::ClientReturningSchema {
                client_id,
                login: "my_name".into(),
                age: 40,
                gender: "Female".into(),
                location: "LA".into(),
            })
        });

        let service = create_test_service();
        let result = service
            .recommendation_ads(
                vec![create_test_campaign(Uuid::new_v4(), Uuid::new_v4())],
                client_id,
                50,
                mock_client_repo,
                MockMlScoreRepo::new(),
            )
            .await;

        assert!(matches!(
            result,
            Err(ServiceError::Repository(RepoError::ObjDoesNotExists(_)))
        ));
    }

    #[tokio::test]
    async fn ml_scores_repository_error_propagated() {
        let client_id = Uuid::new_v4();
        let mut mock_client_repo = MockClientRepo::new();
        mock_client_repo.expect_get_by_id().returning(move |_| {
            Ok(infrastructure::repository::sqlx_lib::ClientReturningSchema {
                client_id,
                login: "my_name".into(),
                age: 25,
                gender: "Male".into(),
                location: "NY".into(),
            })
        });

        let mut mock_ml_repo = MockMlScoreRepo::new();
        mock_ml_repo
            .expect_get_ml_scores()
            .returning(|_, _| Err(RepoError::Unknown));

        let service = create_test_service();
        let result = service
            .recommendation_ads(
                vec![create_test_campaign(Uuid::new_v4(), Uuid::new_v4())],
                client_id,
                50,
                mock_client_repo,
                mock_ml_repo,
            )
            .await;

        assert!(matches!(result, Err(ServiceError::Repository(RepoError::Unknown))));
    }

    #[tokio::test]
    async fn scoring_logic_prioritizes_higher_profit() {
        let client_id = Uuid::new_v4();
        let campaign1_id = Uuid::new_v4();
        let campaign2_id = Uuid::new_v4();

        let mut mock_client_repo = MockClientRepo::new();
        mock_client_repo.expect_get_by_id().returning(move |_| {
            Ok(infrastructure::repository::sqlx_lib::ClientReturningSchema {
                client_id,
                login: "my_name".into(),
                age: 25,
                gender: "Male".into(),
                location: "NY".into(),
            })
        });

        let mut mock_ml_repo = MockMlScoreRepo::new();
        mock_ml_repo.expect_get_ml_scores().returning(|_, _| Ok(vec![0.5, 0.5]));

        let service = AdsService::new(1.0, 0.0, 0.0, 0.0);
        let mut campaign1 = create_test_campaign(campaign1_id, Uuid::new_v4());
        campaign1.cost_per_impression = 10.;

        let mut campaign2 = create_test_campaign(campaign2_id, Uuid::new_v4());
        campaign2.cost_per_impression = 20.;

        let result = service
            .recommendation_ads(
                vec![campaign1, campaign2],
                client_id,
                50,
                mock_client_repo,
                mock_ml_repo,
            )
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().ad_id, campaign2_id);
    }

    #[tokio::test]
    async fn time_left_calculation_affects_score() {
        let client_id = Uuid::new_v4();
        let campaign1_id = Uuid::new_v4();
        let campaign2_id = Uuid::new_v4();

        let mut mock_client_repo = MockClientRepo::new();
        mock_client_repo.expect_get_by_id().returning(move |_| {
            Ok(infrastructure::repository::sqlx_lib::ClientReturningSchema {
                client_id,
                login: "my_name".into(),
                age: 25,
                gender: "Male".into(),
                location: "NY".into(),
            })
        });

        let mut mock_ml_repo = MockMlScoreRepo::new();
        mock_ml_repo.expect_get_ml_scores().returning(|_, _| Ok(vec![0.5, 0.5]));

        let service = AdsService::new(0.0, 0.0, 0.0, 1.0);
        let mut campaign1 = create_test_campaign(campaign1_id, Uuid::new_v4());
        campaign1.end_date = 100;

        let mut campaign2 = create_test_campaign(campaign2_id, Uuid::new_v4());
        campaign2.end_date = 200;

        let result = service
            .recommendation_ads(
                vec![campaign1, campaign2],
                client_id,
                50,
                mock_client_repo,
                mock_ml_repo,
            )
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().ad_id, campaign1_id);
    }

    #[tokio::test]
    async fn client_not_found_error() {
        let client_id = Uuid::new_v4();
        let mut mock_client_repo = MockClientRepo::new();
        mock_client_repo
            .expect_get_by_id()
            .returning(|_| Err(RepoError::ObjDoesNotExists("Client not found".into())));

        let service = create_test_service();
        let result = service
            .recommendation_ads(
                vec![create_test_campaign(Uuid::new_v4(), Uuid::new_v4())],
                client_id,
                50,
                mock_client_repo,
                MockMlScoreRepo::new(),
            )
            .await;

        assert!(matches!(
            result,
            Err(ServiceError::Repository(RepoError::ObjDoesNotExists(_)))
        ));
    }
}
