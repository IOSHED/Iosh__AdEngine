use redis::Commands;

use crate::{domain, infrastructure};

pub struct RedisRepository<'p> {
    pool: &'p infrastructure::database_connection::redis::RedisPool,
}

impl<'p> RedisRepository<'p> {
    pub fn new(pool: &'p infrastructure::database_connection::redis::RedisPool) -> Self {
        RedisRepository { pool }
    }

    #[tracing::instrument(name = "RedisService.set", skip(self, data), level = "debug")]
    pub async fn set<V: redis::ToRedisArgs>(&self, key: &str, data: V) -> domain::services::ServiceResult<()> {
        let mut conn = self.get_conn().await?;

        let _: () = conn
            .set(key, data)
            .map_err(|_| domain::services::ServiceError::Cash("Redis set value error".to_string()))?;

        Ok(())
    }

    #[tracing::instrument(name = "RedisService.get", skip(self), level = "debug")]
    pub async fn get<V: redis::FromRedisValue>(&self, key: &str) -> domain::services::ServiceResult<V> {
        let mut conn = self.get_conn().await?;

        let data: V = conn
            .get(key)
            .map_err(|_| domain::services::ServiceError::Cash("Redis get value error".to_string()))?;

        Ok(data)
    }

    #[tracing::instrument(name = "RedisService.delete", skip(self), level = "debug")]
    pub async fn delete(&self, key: &str) -> domain::services::ServiceResult<()> {
        let mut conn = self.get_conn().await?;

        let _: () = conn
            .del(key)
            .map_err(|_| domain::services::ServiceError::Cash("Redis delete value error".to_string()))?;

        Ok(())
    }

    pub async fn get_conn(&self) -> domain::services::ServiceResult<r2d2::PooledConnection<redis::Client>> {
        self.pool
            .get()
            .await
            .map_err(|_| domain::services::ServiceError::Cash("Redis connection error".to_string()))
    }
}
