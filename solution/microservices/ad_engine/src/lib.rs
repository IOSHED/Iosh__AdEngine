#![allow(clippy::all, clippy::pedantic)]

use infrastructure::{
    configurate,
    database_connection::{sqlx_lib, IGetPoolDatabase},
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
    let pg_pool_creator = sqlx_lib::SqlxPoolCreater::new(config.database.postgres.clone());
    let connection_pool = pg_pool_creator.get_pool().await;
    tracing::info!("Connecting to db complete.");

    // App state init
    let app_state = domain::configurate::AppState::from(&config);
    tracing::info!("App state init complete.");

    // Rabbit server init
    let rabbit_server = interface::lapin::rabbit_client::RabbitServer::new(
        config.rabbit_mq.clone(),
        app_state.clone(),
        connection_pool.clone(),
    );
    tracing::info!("Rabbit server bind complete.");

    // Http client init
    let http_client =
        interface::actix::HttpServer::new(config.http_server, config.cors, app_state, connection_pool).await;
    tracing::info!("Http client bind complete.");

    // Lounches
    let rabbit_launch = rabbit_server.launch();
    let http_launch = http_client.launch();

    let (rabbit_result, http_result) = tokio::try_join!(rabbit_launch, http_launch).expect("Failed to start servers");

    tracing::info!("Rabbit server launch complete: {:?}", rabbit_result);
    tracing::info!("HTTP server launch complete: {:?}", http_result);
}
