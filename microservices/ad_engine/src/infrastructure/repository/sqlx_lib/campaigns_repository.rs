use async_trait::async_trait;
use bigdecimal::FromPrimitive;

use crate::{domain, infrastructure};

#[derive(Debug)]
pub struct PgCampaignRepository<'p> {
    db_pool: &'p sqlx::Pool<sqlx::Postgres>,
}

impl<'p> infrastructure::repository::IRepo<'p> for PgCampaignRepository<'p> {
    fn new(db_pool: &'p sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { db_pool }
    }
}

#[derive(sqlx::FromRow)]
pub struct CampaignReturningSchema {
    pub id: uuid::Uuid,
    pub advertiser_id: uuid::Uuid,
    pub impressions_limit: i64,
    pub clicks_limit: i64,
    pub cost_per_impressions: bigdecimal::BigDecimal,
    pub cost_per_clicks: bigdecimal::BigDecimal,
    pub ad_title: String,
    pub ad_text: String,
    pub start_date: i64,
    pub end_date: i64,
    pub targeting: Option<serde_json::Value>,
}

#[async_trait]
impl<'p> domain::services::repository::ICreateCampaign for PgCampaignRepository<'p> {
    async fn create(
        &self,
        campaign: domain::schemas::CampaignsCreateRequest,
        advertiser_id: uuid::Uuid,
    ) -> infrastructure::repository::RepoResult<CampaignReturningSchema> {
        let campaign = sqlx::query_as!(
            CampaignReturningSchema,
            r#"
            INSERT INTO campaigns (
                advertiser_id,
                impressions_limit,
                clicks_limit,
                cost_per_impressions,
                cost_per_clicks,
                ad_title,
                ad_text,
                start_date,
                end_date,
                targeting
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#,
            advertiser_id,
            campaign.impressions_limit as i32,
            campaign.clicks_limit as i32,
            bigdecimal::BigDecimal::from_f64(campaign.cost_per_impression)
                .ok_or(infrastructure::repository::RepoError::Unknown)?,
            bigdecimal::BigDecimal::from_f64(campaign.cost_per_click)
                .ok_or(infrastructure::repository::RepoError::Unknown)?,
            campaign.ad_title,
            campaign.ad_text,
            campaign.start_date as i32,
            campaign.end_date as i32,
            serde_json::to_value(&campaign.targeting).map_err(|_| infrastructure::repository::RepoError::Unknown)?,
        )
        .fetch_one(self.db_pool)
        .await
        .map_err(|e| {
            if e.to_string()
                .contains("violates foreign key constraint \"campaigns_advertiser_id_fkey\"")
            {
                return infrastructure::repository::RepoError::ObjDoesNotExists("advertiser".to_string());
            }
            e.into()
        })?;

        Ok(campaign)
    }
}

#[async_trait]
impl<'p> domain::services::repository::IUpdateCampaign for PgCampaignRepository<'p> {
    async fn update(
        &self,
        campaign: domain::schemas::CampaignsUpdateRequest,
        advertiser_id: uuid::Uuid,
        campaign_id: uuid::Uuid,
    ) -> infrastructure::repository::RepoResult<CampaignReturningSchema> {
        let campaign = sqlx::query_as!(
            CampaignReturningSchema,
            r#"
            UPDATE campaigns
            SET cost_per_impressions = $1,
                cost_per_clicks = $2,
                ad_title = $3,
                ad_text = $4,
                targeting = $5
            WHERE advertiser_id = $6 AND id = $7
            RETURNING *
            "#,
            bigdecimal::BigDecimal::from_f64(campaign.cost_per_impression)
                .ok_or(infrastructure::repository::RepoError::Unknown)?,
            bigdecimal::BigDecimal::from_f64(campaign.cost_per_click)
                .ok_or(infrastructure::repository::RepoError::Unknown)?,
            campaign.ad_title,
            campaign.ad_text,
            serde_json::to_value(&campaign.targeting).map_err(|_| infrastructure::repository::RepoError::Unknown)?,
            advertiser_id,
            campaign_id
        )
        .fetch_one(self.db_pool)
        .await?;

        Ok(campaign)
    }
}

#[async_trait]
impl<'p> domain::services::repository::IDeleteCampaign for PgCampaignRepository<'p> {
    async fn delete(
        &self,
        advertiser_id: uuid::Uuid,
        campaign_id: uuid::Uuid,
    ) -> infrastructure::repository::RepoResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM campaigns
            WHERE advertiser_id = $1 AND id = $2
            "#,
            advertiser_id,
            campaign_id,
        )
        .execute(self.db_pool)
        .await?;

        Ok(())
    }
}

#[async_trait]
impl<'p> domain::services::repository::IGetCampaignById for PgCampaignRepository<'p> {
    async fn get_by_id(
        &self,
        advertiser_id: uuid::Uuid,
        campaign_id: uuid::Uuid,
    ) -> infrastructure::repository::RepoResult<CampaignReturningSchema> {
        let campaign = sqlx::query_as!(
            CampaignReturningSchema,
            r#"
            SELECT * FROM campaigns
            WHERE advertiser_id = $1 AND id = $2
            "#,
            advertiser_id,
            campaign_id,
        )
        .fetch_one(self.db_pool)
        .await?;

        Ok(campaign)
    }
}

#[async_trait]
impl<'p> domain::services::repository::ISearchCampaign for PgCampaignRepository<'p> {
    async fn are_exist(&self, campaign_id: uuid::Uuid) -> infrastructure::repository::RepoResult<bool> {
        let exists: Option<bool> = sqlx::query_scalar!(
            r#"
            SELECT EXISTS (
                SELECT 1 FROM campaigns WHERE id = $1
            )
            "#,
            campaign_id
        )
        .fetch_one(self.db_pool)
        .await?;

        Ok(exists.unwrap_or(false))
    }
}

#[async_trait]
impl<'p> domain::services::repository::IGetCampaignList for PgCampaignRepository<'p> {
    async fn get_list(
        &self,
        advertiser_id: uuid::Uuid,
        size: u32,
        page: u32,
    ) -> infrastructure::repository::RepoResult<(u64, Vec<CampaignReturningSchema>)> {
        let campaigns: Vec<CampaignReturningSchema> = if size == 0 || page == 0 {
            Vec::new()
        } else {
            sqlx::query_as!(
                CampaignReturningSchema,
                r#"
                SELECT * FROM campaigns
                WHERE advertiser_id = $1
                LIMIT $2 OFFSET $3
                "#,
                advertiser_id,
                size as i32,
                ((page - 1) * size) as i32
            )
            .fetch_all(self.db_pool)
            .await?
        };

        let mut total_count = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) FROM campaigns
            WHERE advertiser_id = $1
            "#,
            advertiser_id,
        )
        .fetch_one(self.db_pool)
        .await?;

        Ok((*total_count.get_or_insert(0) as u64, campaigns))
    }
}

#[async_trait]
impl<'p> domain::services::repository::IGetActiveCampaignList for PgCampaignRepository<'p> {
    async fn get_active_campaigns(
        &self,
        current_date: u32,
    ) -> infrastructure::repository::RepoResult<Vec<CampaignReturningSchema>> {
        let campaign = sqlx::query_as!(
            CampaignReturningSchema,
            r#"
            SELECT * FROM campaigns
            WHERE start_date <= $1 AND end_date >= $1
            "#,
            current_date as i64
        )
        .fetch_all(self.db_pool)
        .await?;

        Ok(campaign.into_iter().map(|c| c.into()).collect())
    }
}

#[async_trait]
impl<'p> domain::services::repository::IGetOrCreateUniqIdForStatCampaign for PgCampaignRepository<'p> {
    async fn get_or_create_uniq_id(
        &self,
        campaign_id: uuid::Uuid,
    ) -> infrastructure::repository::RepoResult<(Vec<uuid::Uuid>, Vec<uuid::Uuid>)> {
        let view_clients: Vec<uuid::Uuid> = sqlx::query_scalar!(
            r#"
            SELECT client_id FROM views_clients
            WHERE campaign_id = $1
            "#,
            campaign_id
        )
        .fetch_all(self.db_pool)
        .await?;

        let click_clients: Vec<uuid::Uuid> = sqlx::query_scalar!(
            r#"
            SELECT client_id FROM clicks_clients
            WHERE campaign_id = $1
            "#,
            campaign_id
        )
        .fetch_all(self.db_pool)
        .await?;

        Ok((view_clients, click_clients))
    }
}

#[async_trait]
impl<'p> domain::services::repository::IViewCampaign for PgCampaignRepository<'p> {
    async fn view_campaign(
        &self,
        campaign_id: uuid::Uuid,
        client_id: uuid::Uuid,
        cost: f64,
        advanced_time: u32,
    ) -> infrastructure::repository::RepoResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO views_clients (campaign_id, client_id, cost, advanced_time)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (campaign_id, client_id) DO NOTHING
            "#,
            campaign_id,
            client_id,
            bigdecimal::BigDecimal::from_f64(cost).ok_or(infrastructure::repository::RepoError::Unknown)?,
            advanced_time as i64
        )
        .execute(self.db_pool)
        .await?;
        Ok(())
    }
}

#[async_trait]
impl<'p> domain::services::repository::IClickCampaign for PgCampaignRepository<'p> {
    async fn click_campaign(
        &self,
        campaign_id: uuid::Uuid,
        client_id: uuid::Uuid,
        cost: f64,
        advanced_time: u32,
    ) -> infrastructure::repository::RepoResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO clicks_clients (campaign_id, client_id, cost, advanced_time)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (campaign_id, client_id) DO NOTHING
            "#,
            campaign_id,
            client_id,
            bigdecimal::BigDecimal::from_f64(cost).unwrap(),
            advanced_time as i64
        )
        .execute(self.db_pool)
        .await?;
        Ok(())
    }
}

#[derive(sqlx::FromRow, Clone)]
pub struct StatDailyReturningSchema {
    pub impressions_count: Option<i32>,
    pub clicks_count: Option<i32>,
    pub spent_impressions: Option<bigdecimal::BigDecimal>,
    pub spent_clicks: Option<bigdecimal::BigDecimal>,
    pub date: Option<i32>,
}

#[async_trait]
impl<'p> domain::services::repository::IGetDailyStat for PgCampaignRepository<'p> {
    async fn get_by_day(
        &self,
        campaign_id: uuid::Uuid,
    ) -> infrastructure::repository::RepoResult<Vec<StatDailyReturningSchema>> {
        let stats = sqlx::query_as!(
            StatDailyReturningSchema,
            r#"
            SELECT
                COALESCE(v.advanced_time, c.advanced_time) as "date",
                COALESCE(v.impressions_count, 0) as "impressions_count",
                COALESCE(c.clicks_count, 0) as "clicks_count",
                COALESCE(v.spent_impressions, 0) as "spent_impressions",
                COALESCE(c.spent_clicks, 0) as "spent_clicks"
            FROM
                (SELECT 
                    advanced_time as "advanced_time",
                    COUNT(*)::INTEGER as "impressions_count",
                    SUM(cost) as "spent_impressions"
                FROM views_clients
                WHERE campaign_id = $1
                GROUP BY advanced_time) v
            FULL JOIN
                (SELECT 
                    advanced_time as "advanced_time",
                    COUNT(*)::INTEGER as "clicks_count",
                    SUM(cost) as "spent_clicks"
                FROM clicks_clients
                WHERE campaign_id = $1
                GROUP BY advanced_time) c
            ON v.advanced_time = c.advanced_time
            ORDER BY date
            "#,
            campaign_id
        )
        .fetch_all(self.db_pool)
        .await?;

        Ok(stats)
    }
}

#[async_trait]
impl<'p> domain::services::repository::IGetIdsCampaign for PgCampaignRepository<'p> {
    async fn get_campaign_ids(
        &self,
        advertiser_id: uuid::Uuid,
    ) -> infrastructure::repository::RepoResult<Vec<uuid::Uuid>> {
        let campaign_ids = sqlx::query_scalar!(
            r#"
            SELECT id FROM campaigns
            WHERE advertiser_id = $1
            "#,
            advertiser_id
        )
        .fetch_all(self.db_pool)
        .await?;

        Ok(campaign_ids)
    }
}
