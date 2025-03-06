use async_trait::async_trait;

use crate::{domain, infrastructure};

#[derive(Debug)]
pub struct PgObsceneWordRepository<'p> {
    db_pool: &'p sqlx::Pool<sqlx::Postgres>,
}

impl<'p> infrastructure::repository::IRepo<'p> for PgObsceneWordRepository<'p> {
    fn new(db_pool: &'p sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl<'p> domain::services::repository::IGetAbusiveWords for PgObsceneWordRepository<'p> {
    async fn get_words(&self) -> infrastructure::repository::RepoResult<Vec<String>> {
        let words: Vec<String> = sqlx::query_scalar!(
            r#"
            SELECT word FROM obscene_words;
            "#,
        )
        .fetch_all(self.db_pool)
        .await?;

        Ok(words)
    }
}
