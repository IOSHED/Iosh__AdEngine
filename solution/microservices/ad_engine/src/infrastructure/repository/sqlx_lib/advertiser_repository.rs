use async_trait::async_trait;

use crate::{domain, infrastructure};

#[derive(Debug)]
pub struct PgAdvertiserRepository<'p> {
    pg_pool: &'p sqlx::Pool<sqlx::Postgres>,
}

impl<'p> infrastructure::repository::IRepo<'p> for PgAdvertiserRepository<'p> {
    fn new(pg_pool: &'p sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pg_pool }
    }
}

#[derive(Debug)]
pub struct AdvertiserReturningSchema {
    pub advertiser_id: uuid::Uuid,
    pub name: String,
}

#[async_trait]
impl<'p> domain::services::repository::IRegisterBulkAdvertiser for PgAdvertiserRepository<'p> {
    async fn register(
        &self,
        advertiser_ids: Vec<uuid::Uuid>,
        names: Vec<String>,
    ) -> infrastructure::repository::RepoResult<Vec<infrastructure::repository::sqlx_lib::AdvertiserReturningSchema>>
    {
        let mut transaction = self.pg_pool.begin().await?;

        let advertisers = sqlx::query_as!(
            AdvertiserReturningSchema,
            r#"
            INSERT INTO advertisers (id, name)
            SELECT * FROM UNNEST($1::UUID[], $2::VARCHAR[])
            ON CONFLICT (id)
            DO UPDATE SET name = EXCLUDED.name
            RETURNING id AS advertiser_id, name
            "#,
            &advertiser_ids,
            &names,
        )
        .fetch_all(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(advertisers)
    }
}

#[async_trait]
impl<'p> domain::services::repository::IGetAdvertiserById for PgAdvertiserRepository<'p> {
    async fn get_by_id(
        &self,
        advertiser_id: uuid::Uuid,
    ) -> infrastructure::repository::RepoResult<infrastructure::repository::sqlx_lib::AdvertiserReturningSchema> {
        let advertiser = sqlx::query_as!(
            AdvertiserReturningSchema,
            r#"
            SELECT id AS advertiser_id, name
            FROM advertisers
            WHERE id = $1
            "#,
            advertiser_id
        )
        .fetch_one(self.pg_pool)
        .await?;

        Ok(advertiser)
    }
}

impl From<infrastructure::repository::sqlx_lib::AdvertiserReturningSchema>
    for domain::schemas::AdvertiserProfileSchema
{
    fn from(user: infrastructure::repository::sqlx_lib::AdvertiserReturningSchema) -> Self {
        Self {
            advertiser_id: user.advertiser_id,
            name: user.name,
        }
    }
}
