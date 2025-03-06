use crate::{
    domain,
    infrastructure::{self, repository::IRepo},
};

pub struct ModerateAddListUsecase<'p> {
    moderate_list_service: domain::services::ModerateListService,
    db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
    redis_service: domain::services::RedisService<'p>,
}

impl<'p> ModerateAddListUsecase<'p> {
    pub fn new(
        db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
        redis_pool: &'p infrastructure::database_connection::redis::RedisPool,
    ) -> Self {
        Self {
            moderate_list_service: domain::services::ModerateListService,
            redis_service: domain::services::RedisService::new(redis_pool),
            db_pool,
        }
    }

    pub async fn add_list(self, add_words: Vec<String>) -> domain::services::ServiceResult<()> {
        let mut words = self.redis_service.get_obscene_words().await?;
        let mut add_words: Vec<String> = add_words.into_iter().map(|word| word.to_lowercase()).collect();

        self.moderate_list_service
            .add_list(
                add_words.clone(),
                infrastructure::repository::sqlx_lib::PgModerateListRepository::new(self.db_pool),
            )
            .await?;

        words.append(&mut add_words);
        self.redis_service.set_obscene_words(words).await?;

        Ok(())
    }
}
