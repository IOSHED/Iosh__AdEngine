use crate::{
    domain,
    infrastructure::{self, repository::IRepo},
};

pub struct StatCampaignUsecase<'p> {
    campaign_stat_service: domain::services::CampaignStatService,
    campaign_service: domain::services::CampaignService,
    aggregate_stat_service: domain::services::AggregateStatService,
    db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
}

impl<'p> StatCampaignUsecase<'p> {
    pub fn new(db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool) -> Self {
        Self {
            campaign_stat_service: domain::services::CampaignStatService,
            campaign_service: domain::services::CampaignService,
            aggregate_stat_service: domain::services::AggregateStatService,
            db_pool,
        }
    }

    pub async fn get_with_advertisers(
        &self,
        advertiser_id: uuid::Uuid,
    ) -> domain::services::ServiceResult<domain::schemas::StatResponse> {
        let stat_by_day = self.get_with_advertisers_by_day(advertiser_id).await?;
        let (impressions, clicks, spent_imp, spent_clk) =
            self.aggregate_stat_service.calculate_total_stats(&stat_by_day);

        Ok(self
            .aggregate_stat_service
            .create_stat_response(impressions, clicks, spent_imp, spent_clk))
    }

    pub async fn get_with_advertisers_by_day(
        &self,
        advertiser_id: uuid::Uuid,
    ) -> domain::services::ServiceResult<Vec<domain::schemas::StatDailyResponse>> {
        let repo = infrastructure::repository::sqlx_lib::PgCampaignRepository::new(self.db_pool);
        let campaign_ids = self.campaign_service.get_campaign_ids(advertiser_id, repo).await?;

        let stats = futures::future::join_all(campaign_ids.into_iter().map(|id| self.get_by_day(id)))
            .await
            .into_iter()
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        Ok(self.aggregate_stat_service.aggregate_daily_stats(stats))
    }

    pub async fn get(&self, campaign_id: uuid::Uuid) -> domain::services::ServiceResult<domain::schemas::StatResponse> {
        let stat_by_day = self.get_by_day(campaign_id).await?;
        let (impressions, clicks, spent_imp, spent_clk) =
            self.aggregate_stat_service.calculate_total_stats(&stat_by_day);

        Ok(self
            .aggregate_stat_service
            .create_stat_response(impressions, clicks, spent_imp, spent_clk))
    }

    pub async fn get_by_day(
        &self,
        campaign_id: uuid::Uuid,
    ) -> domain::services::ServiceResult<Vec<domain::schemas::StatDailyResponse>> {
        let repo = infrastructure::repository::sqlx_lib::PgCampaignRepository::new(self.db_pool);
        self.campaign_stat_service.get_by_day(campaign_id, repo).await
    }
}
