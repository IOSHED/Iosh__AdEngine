use async_trait::async_trait;
use bigdecimal::ToPrimitive;

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
        cost: f64,
        advanced_time: u32,
    ) -> infrastructure::repository::RepoResult<()>;
}

#[async_trait]
pub trait IClickCampaign {
    async fn click_campaign(
        &self,
        campaign_id: uuid::Uuid,
        client_id: uuid::Uuid,
        cost: f64,
        advanced_time: u32,
    ) -> infrastructure::repository::RepoResult<()>;
}

#[async_trait]
pub trait IGetDailyStat {
    async fn get_by_day(
        &self,
        campaign_id: uuid::Uuid,
    ) -> infrastructure::repository::RepoResult<Vec<infrastructure::repository::sqlx_lib::StatDailyReturningSchema>>;
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
        cost: f64,
        advanced_time: u32,
        repo: R,
    ) -> domain::services::ServiceResult<()> {
        repo.view_campaign(campaign_id, client_id, cost, advanced_time)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))
    }

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

impl From<infrastructure::repository::sqlx_lib::StatDailyReturningSchema> for domain::schemas::StatDailyResponse {
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
