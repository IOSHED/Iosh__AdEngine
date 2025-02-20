use crate::{domain, infrastructure};

pub struct ModerateGetListUsecase<'p> {
    moderate_list_service: domain::services::ModerateListService,
    redis_pool: &'p infrastructure::database_connection::redis::RedisPool,
    db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
}

impl<'p> ModerateGetListUsecase<'p> {
    pub fn new(
        redis_pool: &'p infrastructure::database_connection::redis::RedisPool,
        db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
    ) -> Self {
        Self {
            redis_pool,
            db_pool,
            moderate_list_service: domain::services::ModerateListService,
        }
    }

    pub async fn get_list(self) -> domain::services::ServiceResult<Vec<String>> {
        let words = self
            .moderate_list_service
            .get_list(infrastructure::repository::redis::RedisObsceneWordRepository::new(
                self.redis_pool,
                self.db_pool,
            ))
            .await?;

        Ok(words)
    }
}
