use async_trait::async_trait;

use crate::{domain, infrastructure};

#[derive(Debug)]
pub struct PgModerateListRepository<'p> {
    db_pool: &'p sqlx::Pool<sqlx::Postgres>,
}

impl<'p> infrastructure::repository::IRepo<'p> for PgModerateListRepository<'p> {
    fn new(db_pool: &'p sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl<'p> domain::services::repository::IAddModerateList for PgModerateListRepository<'p> {
    async fn add_list(&self, add_words: Vec<String>) -> infrastructure::repository::RepoResult<()> {
        let mut transaction = self.db_pool.begin().await?;

        sqlx::query!(
            r#"
            INSERT INTO obscene_words (word)
            SELECT * FROM UNNEST($1::VARCHAR[]);
            "#,
            &add_words
        )
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }
}

#[async_trait]
impl<'p> domain::services::repository::IDeleteModerateList for PgModerateListRepository<'p> {
    async fn delete_list(&self, delete_words: Vec<String>) -> infrastructure::repository::RepoResult<()> {
        let mut transaction = self.db_pool.begin().await?;

        sqlx::query_as!(
            ClientReturningSchema,
            r#"
            DELETE FROM obscene_words
            WHERE word = ANY($1::VARCHAR[]);
            "#,
            &delete_words
        )
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }
}
