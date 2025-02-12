use async_trait::async_trait;

use super::IGetPoolDatabase;
use crate::configurate::PostgresConfig;

/// Type alias for the database connection pool used by SQLx
pub type SqlxPool = sqlx::Pool<sqlx::Postgres>;

/// Creates and manages a SQLx connection pool for Postgres databases
pub struct SqlxPoolCreater {
    /// Configuration for the Postgres database connection
    config: PostgresConfig,
}

impl SqlxPoolCreater {
    /// Creates a new SqlxPoolCreater with the given Postgres configuration
    pub fn new(config: PostgresConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl IGetPoolDatabase for SqlxPoolCreater {
    /// The type of database pool that will be created (SQLx Postgres pool)
    type Pool = SqlxPool;

    /// Creates and returns a new pool of database connections by running
    /// migrations
    ///
    /// # Returns
    /// A SQLx connection pool configured for Postgres
    ///
    /// # Panics
    /// Will panic if unable to establish database connection
    async fn get_pool(&self) -> Self::Pool {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(self.config.max_connections)
            .connect(&self.config.postgres_conn)
            .await
            .expect("Failed to connect database");

        let err_migration = sqlx::migrate!()
            .run(&pool)
            .await
            .map_err(|e| format!("Failed migrations! Migrations failed :( {e}"))
            .err();

        if let Some(err) = err_migration {
            tracing::error!("{:?}", err);
        }

        pool
    }
}
