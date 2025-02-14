use rand::Rng;

use crate::{domain, infrastructure};

#[derive(Debug)]
pub struct AdsService;

impl AdsService {
    pub async fn recommendation_ads<R1, R2>(
        &self,
        active_campaigns: Vec<domain::schemas::ActiveCampaignSchema>,
        client_id: uuid::Uuid,
        repo_client: R1,
        repo_score: R2,
    ) -> domain::services::ServiceResult<domain::schemas::AdSchema>
    where
        R1: super::repository::IGetClientById,
        R2: super::repository::IGetMlScore,
    {
        let client = self.get_client(repo_client, client_id).await?;

        let suitable_campaigns = self.get_suitable_campaigns(active_campaigns, &client).await?;

        let scored_campaigns = self.score_campaigns(suitable_campaigns, client_id, &repo_score).await?;

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
        let suitable_campaigns = if self.should_show_non_targeted().await {
            active_campaigns
        } else {
            self.filter_targeted_campaigns(
                active_campaigns,
                client.age as u8,
                client.gender.clone(),
                client.location.clone(),
            )
            .await
        };

        let filtered_campaigns: Vec<_> = suitable_campaigns
            .into_iter()
            .filter(|c| !c.click_clients_id.contains(&client.client_id))
            .filter(|c| (c.view_clients_id.len() as u32) < c.impressions_limit)
            .collect();

        if filtered_campaigns.is_empty() {
            return Err(domain::services::ServiceError::Validation(
                "No suitable campaigns found".into(),
            ));
        }

        Ok(filtered_campaigns)
    }

    async fn score_campaigns<R>(
        &self,
        suitable_campaigns: Vec<domain::schemas::ActiveCampaignSchema>,
        client_id: uuid::Uuid,
        repo_score: &R,
    ) -> domain::services::ServiceResult<Vec<(f64, u32, domain::schemas::ActiveCampaignSchema)>>
    where
        R: super::repository::IGetMlScore,
    {
        let mut scored_campaigns =
            futures::future::join_all(suitable_campaigns.into_iter().map(|campaign| async move {
                let remaining_impressions = campaign.impressions_limit as f64 - campaign.view_clients_id.len() as f64;
                let remaining_clicks = campaign.clicks_limit as f64 - campaign.click_clients_id.len() as f64;

                let profit = (remaining_impressions * campaign.cost_per_impressions as f64)
                    + (remaining_clicks * campaign.cost_per_clicks as f64);

                let relevance = repo_score
                    .get_ml_score(client_id, campaign.campaign_id)
                    .await
                    .unwrap_or(0.0);

                let fulfillment = (remaining_impressions / campaign.impressions_limit as f64)
                    + (remaining_clicks / campaign.clicks_limit as f64);

                let combined_score = 0.5 * profit + 0.25 * relevance + 0.15 * fulfillment;

                (combined_score, campaign.end_date, campaign)
            }))
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
                c.targeting.location == location
                    && (c.targeting.gender == gender || c.targeting.gender == "ALL")
                    && c.targeting.age_from <= age
                    && age <= c.targeting.age_to
            })
            .collect()
    }

    async fn should_show_non_targeted(&self) -> bool {
        let mut rng = rand::rng();
        rng.random_range(0..100) < 4
    }
}
