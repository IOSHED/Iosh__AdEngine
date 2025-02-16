use async_trait::async_trait;

use crate::{domain, infrastructure};

#[async_trait]
pub trait IGetMlScores {
    async fn get_ml_scores(
        &self,
        client_id: uuid::Uuid,
        advertisers_id: Vec<uuid::Uuid>,
    ) -> infrastructure::repository::RepoResult<Vec<f64>>;
}

#[derive(Debug)]
pub struct AdsService {
    weight_profit: f64,
    weight_relevance: f64,
    weight_fulfillment: f64,
    weight_time_left: f64,
}

impl AdsService {
    pub fn new(weight_profit: f64, weight_relevance: f64, weight_fulfillment: f64, weight_time_left: f64) -> Self {
        Self {
            weight_profit,
            weight_relevance,
            weight_fulfillment,
            weight_time_left,
        }
    }
}

impl AdsService {
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
            .map_err(domain::services::ServiceError::Repository)
    }

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
            .filter(|c| !c.view_clients_id.contains(&client.client_id))
            .filter(|c| (c.view_clients_id.len() as u32) <= c.impressions_limit)
            .collect::<Vec<_>>();

        if filtered_campaigns.is_empty() {
            return Err(domain::services::ServiceError::Repository(
                infrastructure::repository::RepoError::ObjDoesNotExists("Suitable campaigns".into()),
            ));
        }

        Ok(filtered_campaigns)
    }

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
            .iter()
            .map(|campaign| {
                let remaining_impressions = campaign.impressions_limit as f64 - campaign.view_clients_id.len() as f64;
                let remaining_clicks = campaign.clicks_limit as f64 - campaign.click_clients_id.len() as f64;
                (remaining_impressions * campaign.cost_per_impressions as f64)
                    + (remaining_clicks * campaign.cost_per_clicks as f64)
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

    async fn get_top_campaign<'a>(
        &self,
        scored_campaigns: &'a [(f64, u32, domain::schemas::ActiveCampaignSchema)],
    ) -> domain::services::ServiceResult<&'a domain::schemas::ActiveCampaignSchema> {
        scored_campaigns
            .get(0)
            .map(|(_, _, campaign)| campaign)
            .ok_or_else(|| domain::services::ServiceError::Validation("No top campaign found".into()))
    }

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

    fn calculate_min_max(&self, values: &[f64]) -> (f64, f64) {
        let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        (min, max)
    }

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

    fn normalize_value(&self, value: f64, min: f64, max: f64) -> f64 {
        if max != min {
            return ((value + 1.0).ln() - (min + 1.0).ln()) / ((max + 1.0).ln() - (min + 1.0).ln());
        }
        0.0
    }

    fn calculate_fulfillment(&self, campaign: &domain::schemas::ActiveCampaignSchema) -> f64 {
        let remaining_impressions = campaign.impressions_limit as f64 - campaign.view_clients_id.len() as f64;
        let remaining_clicks = campaign.clicks_limit as f64 - campaign.click_clients_id.len() as f64;
        (remaining_impressions / campaign.impressions_limit as f64) + (remaining_clicks / campaign.clicks_limit as f64)
    }

    fn calculate_time_left(&self, end_date: u32, advanced_time: u32) -> f64 {
        1.0 - ((end_date as f64 - advanced_time as f64) / u32::MAX as f64).clamp(0.0, 1.0)
    }
}
