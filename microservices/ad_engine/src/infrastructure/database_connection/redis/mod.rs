use crate::infrastructure;

/// Redis connection pool wrapper for managing Redis connections.
///
/// This struct provides a thread-safe connection pool for Redis using r2d2.
/// It handles connection management and pooling automatically.
#[derive(Debug, Clone)]
pub struct RedisPool {
    /// The underlying r2d2 connection pool for Redis clients
    pool: r2d2::Pool<redis::Client>,
}

impl RedisPool {
    /// Creates a new Redis connection pool using the provided configuration.
    ///
    /// # Arguments
    /// * `config` - Redis configuration containing host, port and database
    ///   details
    ///
    /// # Returns
    /// A new `RedisPool` instance with an initialized connection pool
    ///
    /// # Panics
    /// - If Redis client creation fails due to invalid URL
    /// - If pool building fails
    pub async fn new(config: infrastructure::configurate::RedisConfig) -> Self {
        let url = format!("redis://{}:{}/{}", config.host, config.port, config.db);
        let client = redis::Client::open(url).expect("Failed starting Redis. Not found url.");
        let pool = r2d2::Pool::builder()
            .build(client)
            .expect("Failed starting Redis. Not found pool.");

        Self { pool }
    }

    /// Retrieves a connection from the pool.
    ///
    /// # Returns
    /// A Result containing either:
    /// - Ok(PooledConnection): A pooled Redis connection that will
    ///   automatically return to the pool when dropped
    /// - Err(Error): An error if connection acquisition fails
    pub async fn get(&self) -> Result<r2d2::PooledConnection<redis::Client>, r2d2::Error> {
        self.pool.get()
    }
}
