use async_trait::async_trait;

use crate::{domain, infrastructure};

#[derive(Debug)]
pub struct PgScoreRepository<'p> {
    pg_pool: &'p sqlx::Pool<sqlx::Postgres>,
}

impl<'p> PgScoreRepository<'p> {
    pub fn new(pg_pool: &'p sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pg_pool }
    }
}

#[async_trait]
impl<'p> domain::services::repository::ISetMlScore for PgScoreRepository<'p> {
    async fn set_ml_score(
        &self,
        client_id: uuid::Uuid,
        advertiser_id: uuid::Uuid,
        score: f64,
    ) -> infrastructure::repository::RepoResult<()> {
        sqlx::query!(
            "
            INSERT INTO ml_scores (client_id, advertiser_id, score)
            VALUES ($1, $2, $3)
            ON CONFLICT (client_id, advertiser_id)
            DO UPDATE SET score = EXCLUDED.score
            WHERE EXISTS (SELECT 1 FROM clients WHERE id = $1)
              AND EXISTS (SELECT 1 FROM advertisers WHERE id = $2)
            ",
            client_id,
            advertiser_id,
            score,
        )
        .fetch_one(self.pg_pool)
        .await?;

        Ok(())
    }
}
