use redis::Commands;

use crate::{domain, infrastructure};

pub struct RedisService<'p> {
    pool: &'p infrastructure::database_connection::redis::RedisPool,
}

impl<'p> RedisService<'p> {
    pub fn new(pool: &'p infrastructure::database_connection::redis::RedisPool) -> Self {
        RedisService { pool }
    }

    pub async fn get_advance_time<V: redis::FromRedisValue>(&self) -> domain::services::ServiceResult<V> {
        match self.get("advance_time").await {
            Ok(data) => Ok(data),
            Err(_) => {
                self.set_advance_time(0 as u32).await?;
                self.get("advance_time").await
            },
        }
    }

    pub async fn set_advance_time<V: redis::ToRedisArgs>(&self, data: V) -> domain::services::ServiceResult<()> {
        self.set("advance_time", data).await
    }

    #[tracing::instrument(name = "RedisService.set", skip(self, data), level = "debug")]
    async fn set<V: redis::ToRedisArgs>(&self, key: &str, data: V) -> domain::services::ServiceResult<()> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|_| domain::services::ServiceError::Cash("Redis connection error".to_string()))?;

        let _: () = conn
            .set(key, data)
            .map_err(|_| domain::services::ServiceError::Cash("Redis set value error".to_string()))?;

        Ok(())
    }

    #[tracing::instrument(name = "RedisService.get", skip(self), level = "debug")]
    async fn get<V: redis::FromRedisValue>(&self, key: &str) -> domain::services::ServiceResult<V> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|_| domain::services::ServiceError::Cash("Redis connection error".to_string()))?;

        let data: V = conn
            .get(key)
            .map_err(|_| domain::services::ServiceError::Cash("Redis get value error".to_string()))?;

        Ok(data)
    }
}
