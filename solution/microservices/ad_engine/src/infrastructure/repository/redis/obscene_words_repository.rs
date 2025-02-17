use async_trait::async_trait;

use crate::{
    domain,
    infrastructure::{self, repository::IRepo},
};

pub struct RedisObsceneWordRepository<'p> {
    repo: infrastructure::cash::redis::RedisExecutor<'p>,
    db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
}

impl<'p> RedisObsceneWordRepository<'p> {
    pub fn new(
        redis_pool: &'p infrastructure::database_connection::redis::RedisPool,
        db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
    ) -> Self {
        Self {
            repo: infrastructure::cash::redis::RedisExecutor::new(redis_pool),
            db_pool,
        }
    }
}

#[async_trait]
impl<'p> domain::services::repository::IGetAbusiveWords for RedisObsceneWordRepository<'p> {
    async fn get_words(&self) -> infrastructure::repository::RepoResult<Vec<String>> {
        let query_res: Result<String, _> = self.repo.get("obscene_words").await;
        match query_res {
            Ok(query_res) => Ok(query_res.split(",").map(|s| s.to_string()).collect()),
            Err(_) => {
                let words: Vec<String> =
                    infrastructure::repository::sqlx_lib::PgObsceneWordRepository::new(self.db_pool)
                        .get_words()
                        .await?;

                let words_string: String = words.clone().join(",");

                self.repo.set("obscene_words", words_string).await.map_err(|e| {
                    tracing::error!("Error while setting obscene words to redis: {}", e);
                    infrastructure::repository::RepoError::Unknown
                })?;

                return Ok(words);
            },
        }
    }
}
