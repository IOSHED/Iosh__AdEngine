use async_trait::async_trait;

use crate::{domain, infrastructure};

#[derive(Debug)]
pub struct PgAdvertiserRepository<'p> {
    pg_pool: &'p sqlx::Pool<sqlx::Postgres>,
}

impl<'p> PgAdvertiserRepository<'p> {
    pub fn new(pg_pool: &'p sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pg_pool }
    }
}

#[derive(Debug)]
pub struct AdvertiserReturningSchema {
    pub advertiser_id: String,
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
        let advertisers = sqlx::query_as!(
            AdvertiserReturningSchema,
            "
            INSERT INTO advertisers (id, name)
            SELECT * FROM UNNEST($1::UUID[], $2::VARCHAR[])
            ON CONFLICT (id) 
            DO UPDATE SET name = EXCLUDED.name
            RETURNING id AS advertiser_id, name
            ",
            &advertiser_ids,
            &names,
        )
        .fetch_all(self.pg_pool)
        .await?;

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
            "
            SELECT id AS advertiser_id, name
            FROM advertisers
            WHERE id = $1
            ",
            advertiser_id
        )
        .fetch_one(self.pg_pool)
        .await?;

        Ok(advertiser)
    }
}
