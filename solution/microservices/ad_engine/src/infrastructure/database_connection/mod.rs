use async_trait::async_trait;

pub mod redis;
pub mod sqlx_lib;

#[async_trait]
pub trait IGetPoolDatabase: Send + Sync {
    type Pool: Send + Sync;

    async fn get_pool(&self) -> Self::Pool;
}
