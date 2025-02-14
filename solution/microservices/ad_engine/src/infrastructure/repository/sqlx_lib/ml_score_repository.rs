use async_trait::async_trait;

use crate::{domain, infrastructure};

#[derive(Debug)]
pub struct PgScoreRepository<'p> {
    db_pool: &'p sqlx::Pool<sqlx::Postgres>,
}

impl<'p> infrastructure::repository::IRepo<'p> for PgScoreRepository<'p> {
    fn new(db_pool: &'p sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl<'p> domain::services::repository::IGetMlScore for PgScoreRepository<'p> {
    async fn get_ml_score(
        &self,
        client_id: uuid::Uuid,
        advertiser_id: uuid::Uuid,
    ) -> infrastructure::repository::RepoResult<f64> {
        let score = sqlx::query_scalar!(
            r#"
            SELECT score FROM ml_scores WHERE client_id = $1 AND advertiser_id = $2
            "#,
            client_id,
            advertiser_id,
        )
        .fetch_optional(self.db_pool)
        .await?;

        Ok(score.unwrap_or(0.))
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
        let mut transaction = self.db_pool.begin().await?;

        let exists: Option<bool> = sqlx::query_scalar!(
            r#"
            SELECT EXISTS (
                SELECT 1 FROM clients WHERE id = $1
            ) AND EXISTS (
                SELECT 1 FROM advertisers WHERE id = $2
            )
            "#,
            client_id,
            advertiser_id,
        )
        .fetch_one(&mut *transaction)
        .await?;

        if !exists.unwrap_or(false) {
            return Err(infrastructure::repository::RepoError::ObjDoesNotExists(
                "Client or Advertiser ID does not exist".into(),
            ));
        }

        sqlx::query!(
            r#"
            INSERT INTO ml_scores (client_id, advertiser_id, score)
            VALUES ($1, $2, $3)
            ON CONFLICT (client_id, advertiser_id)
            DO UPDATE SET score = EXCLUDED.score
            "#,
            client_id,
            advertiser_id,
            score,
        )
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }
}
