use crate::{domain, infrastructure};

pub struct ModerateGetListUsecase<'p> {
    redis_service: domain::services::RedisService<'p>,
}

impl<'p> ModerateGetListUsecase<'p> {
    pub fn new(redis_pool: &'p infrastructure::database_connection::redis::RedisPool) -> Self {
        Self {
            redis_service: domain::services::RedisService::new(redis_pool),
        }
    }

    pub async fn get_list(self) -> domain::services::ServiceResult<Vec<String>> {
        let words = self.redis_service.get_obscene_words().await?;

        Ok(words)
    }
}
