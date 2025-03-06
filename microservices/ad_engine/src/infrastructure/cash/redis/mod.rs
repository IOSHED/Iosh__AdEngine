use redis::Commands;

use crate::{domain, infrastructure};

/// Redis executor that provides high-level operations for interacting with
/// Redis cache
///
/// Wraps a connection pool and provides async methods for common Redis
/// operations like get, set and delete. Handles connection management and error
/// mapping.
pub struct RedisExecutor<'p> {
    pool: &'p infrastructure::database_connection::redis::RedisPool,
}

impl<'p> RedisExecutor<'p> {
    /// Creates a new RedisExecutor with the provided connection pool
    ///
    /// # Arguments
    /// * `pool` - Reference to a Redis connection pool that will be used for
    ///   all operations
    pub fn new(pool: &'p infrastructure::database_connection::redis::RedisPool) -> Self {
        RedisExecutor { pool }
    }

    /// Sets a value in Redis for the given key
    ///
    /// # Arguments
    /// * `key` - The key under which to store the value
    /// * `data` - The value to store, must implement ToRedisArgs
    ///
    /// # Returns
    /// * `Ok(())` if the operation succeeds
    /// * `Err(ServiceError::Cash)` if the Redis operation fails
    #[tracing::instrument(name = "RedisService.set", skip(self, data), level = "debug")]
    pub async fn set<V: redis::ToRedisArgs>(&self, key: &str, data: V) -> domain::services::ServiceResult<()> {
        let mut conn = self.get_conn().await?;

        let _: () = conn
            .set(key, data)
            .map_err(|_| domain::services::ServiceError::Cash("Redis set value error".to_string()))?;

        Ok(())
    }

    /// Retrieves a value from Redis by key
    ///
    /// # Arguments
    /// * `key` - The key whose value should be retrieved
    ///
    /// # Returns
    /// * `Ok(V)` with the retrieved value if successful
    /// * `Err(ServiceError::Cash)` if the Redis operation fails or value cannot
    ///   be deserialized
    #[tracing::instrument(name = "RedisService.get", skip(self), level = "debug")]
    pub async fn get<V: redis::FromRedisValue>(&self, key: &str) -> domain::services::ServiceResult<V> {
        let mut conn = self.get_conn().await?;

        let data: V = conn
            .get(key)
            .map_err(|_| domain::services::ServiceError::Cash("Redis get value error".to_string()))?;

        Ok(data)
    }

    /// Deletes a key and its associated value from Redis
    ///
    /// # Arguments
    /// * `key` - The key to delete
    ///
    /// # Returns
    /// * `Ok(())` if the deletion succeeds
    /// * `Err(ServiceError::Cash)` if the Redis operation fails
    #[tracing::instrument(name = "RedisService.delete", skip(self), level = "debug")]
    pub async fn delete(&self, key: &str) -> domain::services::ServiceResult<()> {
        let mut conn = self.get_conn().await?;

        let _: () = conn
            .del(key)
            .map_err(|_| domain::services::ServiceError::Cash("Redis delete value error".to_string()))?;

        Ok(())
    }

    /// Gets a connection from the pool
    ///
    /// # Returns
    /// * `Ok(PooledConnection)` with a connection if one is available
    /// * `Err(ServiceError::Cash)` if unable to get a connection
    pub async fn get_conn(&self) -> domain::services::ServiceResult<r2d2::PooledConnection<redis::Client>> {
        self.pool
            .get()
            .await
            .map_err(|_| domain::services::ServiceError::Cash("Redis connection error".to_string()))
    }
}
