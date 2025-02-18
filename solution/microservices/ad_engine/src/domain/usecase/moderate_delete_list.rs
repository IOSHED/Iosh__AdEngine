use crate::{
    domain,
    infrastructure::{self, repository::IRepo},
};

pub struct ModerateDeleteListUsecase<'p> {
    moderate_list_service: domain::services::ModerateListService,
    db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
    redis_service: domain::services::RedisService<'p>,
}

impl<'p> ModerateDeleteListUsecase<'p> {
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

    pub async fn delete_list(self, delete_words: Vec<String>) -> domain::services::ServiceResult<()> {
        let words = self.redis_service.get_obscene_words().await?;
        let delete_words: Vec<String> = delete_words.into_iter().map(|word| word.to_lowercase()).collect();

        self.moderate_list_service
            .delete_list(
                delete_words.clone(),
                infrastructure::repository::sqlx_lib::PgModerateListRepository::new(self.db_pool),
            )
            .await?;

        let filtered_words: Vec<String> = words.into_iter().filter(|w| !delete_words.contains(w)).collect();

        self.redis_service.set_obscene_words(filtered_words).await?;

        Ok(())
    }
}
