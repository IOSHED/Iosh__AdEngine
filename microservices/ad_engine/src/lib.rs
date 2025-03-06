#![allow(clippy::all, clippy::pedantic)]

use infrastructure::{
    configurate,
    database_connection::{self, IGetPoolDatabase},
    logger::tracing_lib,
};
use interface::IServer;

mod domain;
mod infrastructure;
mod interface;

pub async fn startapp() {
    // Config init
    let config = configurate::parse(std::path::PathBuf::from("./conf")).await;

    // Logger init
    tracing_lib::setup_logging(config.logger.clone()).await;
    tracing::info!("Logging setup complete.");

    // Database Postgres connect init
    let pg_pool_creator = database_connection::sqlx_lib::SqlxPoolCreater::new(config.database.postgres.clone());
    let connection_pool = pg_pool_creator.get_pool().await;
    tracing::info!("Connecting to db complete.");

    // Redis init
    let redis_pool = database_connection::redis::RedisPool::new(config.database.redis.clone()).await;

    // App state init
    let app_state = domain::configurate::AppState::from(&config);
    tracing::info!("App state init complete.");

    // Http client init
    let http_client =
        interface::actix::HttpServer::new(config.http_server, config.cors, app_state, connection_pool, redis_pool)
            .await;
    tracing::info!("Http client bind complete.");

    let http_launch = http_client.launch().await;
    tracing::info!("HTTP server launch complete: {:#?}", http_launch);
}
