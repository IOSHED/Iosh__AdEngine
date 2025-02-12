use crate::infrastructure;

#[derive(Debug, Clone)]
pub struct RedisPool {
    pool: r2d2::Pool<redis::Client>,
}

impl RedisPool {
    pub async fn new(config: infrastructure::configurate::RedisConfig) -> Self {
        let url = format!("redis://{}:{}/{}", config.host, config.port, config.db);
        let client = redis::Client::open(url).expect("Failed starting Redis. Not found url.");
        let pool = r2d2::Pool::builder()
            .build(client)
            .expect("Failed starting Redis. Not found pool.");

        Self { pool }
    }

    pub async fn get(&self) -> Result<r2d2::PooledConnection<redis::Client>, r2d2::Error> {
        self.pool.get()
    }
}
