//! HTTP Server implementation for handling web requests
//!
//! This module provides the core HTTP server functionality including:
//! - Server configuration and setup
//! - Database connection pooling
//! - JSON request handling
//! - API routing
//! - Swagger documentation
//! - CORS configuration
//! - Server lifecycle management

use async_trait::async_trait;
use utoipa::OpenApi;

mod swagger;

use crate::{domain, infrastructure, interface};

/// Main HTTP server struct that handles web requests
///
/// Generic type parameter D represents the database connection pool
/// implementation
pub struct HttpServer {
    config: infrastructure::configurate::HttpServerConfig,
    cors_config: infrastructure::configurate::CorsConfig,
    app_state: domain::configurate::AppState,
    connection_pool: infrastructure::database_connection::sqlx_lib::SqlxPool,
    redis_pool: infrastructure::database_connection::redis::RedisPool,
}

impl HttpServer {
    /// Creates a new HTTP server instance
    ///
    /// # Arguments
    /// * `config` - Server configuration
    /// * `cors_config` - CORS configuration
    /// * `app_state` - Application state
    /// * `fn_connection_pool` - Database connection pool
    pub async fn new(
        config: infrastructure::configurate::HttpServerConfig,
        cors_config: infrastructure::configurate::CorsConfig,
        app_state: domain::configurate::AppState,
        connection_pool: infrastructure::database_connection::sqlx_lib::SqlxPool,
        redis_pool: infrastructure::database_connection::redis::RedisPool,
    ) -> Self {
        Self {
            config,
            cors_config,
            app_state,
            connection_pool,
            redis_pool,
        }
    }

    /// Gets the database connection pool wrapped in actix web Data
    async fn get_database_pool_data(
        &self,
    ) -> actix_web::web::Data<infrastructure::database_connection::sqlx_lib::SqlxPool> {
        actix_web::web::Data::new(self.connection_pool.clone())
    }

    async fn get_redis_pool_data(&self) -> actix_web::web::Data<infrastructure::database_connection::redis::RedisPool> {
        actix_web::web::Data::new(self.redis_pool.clone())
    }

    /// Configures JSON request handling including size limits and error
    /// handling
    fn get_json_config(&self) -> actix_web::web::JsonConfig {
        actix_web::web::JsonConfig::default()
            .limit(self.config.limit_size_json)
            .error_handler(|err, _req| {
                let status_code = actix_web::http::StatusCode::CONFLICT;

                let exception_response = interface::actix::exception::ExceptionResponse::new(err.to_string());

                actix_web::error::InternalError::from_response(
                    err,
                    actix_web::HttpResponse::build(status_code).json(exception_response),
                )
                .into()
            })
    }

    /// Creates the main API scope with routes
    fn get_api_scope(&self) -> actix_web::Scope {
        actix_web::web::scope("/api")
            .service(super::routers::healthcheck_handler)
            .service(super::routers::time_advance_handler)
            .service(super::routers::ml_score_handler)
            .service(super::routers::ads_handler)
            .service(super::routers::client_scope("/client"))
            .service(super::routers::advertisers_scope("/advertisers"))
    }

    /// Configures Swagger documentation UI
    fn get_swagger_docs(&self) -> utoipa_swagger_ui::SwaggerUi {
        let swagger_path = format!("{}/{}", self.config.path_swagger_docs.display(), "{_:.*}");
        let openapi_path = format!("{}/openapi.json", self.config.path_openapi_docs.display());

        tracing::debug!("Configuring Swagger UI at path: {}", swagger_path);

        utoipa_swagger_ui::SwaggerUi::new(swagger_path).url(openapi_path, swagger::ApiDocSwagger::openapi())
    }

    /// Configures CORS settings for the server
    fn get_cors(&self) -> actix_cors::Cors {
        actix_cors::Cors::default()
            .allowed_origin(&self.cors_config.allowed_origin)
            .allowed_methods(self.cors_config.allowed_methods.iter().map(|str_method| {
                actix_web::http::Method::from_bytes(str_method.as_bytes())
                    .expect("Config contain not verifying http method (cors.allowed_methods)")
            }))
            .allowed_headers(self.cors_config.allowed_headers.clone())
            .max_age(self.cors_config.max_age)
    }
}

#[async_trait]
impl interface::IServer for HttpServer {
    type ErrorLaunch = Box<dyn std::error::Error>;

    /// Launches the HTTP server
    ///
    /// Sets up middleware, routes and starts listening for requests
    async fn launch(self) -> Result<(), Self::ErrorLaunch> {
        let server = std::sync::Arc::new(self);

        let config = &server.clone().config;

        let app_state = actix_web::web::Data::new(server.app_state.clone());

        let database_pool_data = server.get_database_pool_data().await;
        let redis_pool_data = server.get_redis_pool_data().await;

        let server = actix_web::HttpServer::new(move || {
            actix_web::App::new()
                .wrap(tracing_actix_web::TracingLogger::default())
                .wrap(actix_web::middleware::Compress::default())
                .wrap(server.get_cors())
                .app_data(server.get_json_config())
                .app_data(redis_pool_data.clone())
                .app_data(database_pool_data.clone())
                .app_data(app_state.clone())
                .service(server.get_api_scope())
                .service(server.get_swagger_docs())
        });

        let server = server
            .bind((config.host, config.port))?
            .shutdown_timeout(config.timeout_shutdown_workers.as_secs())
            .keep_alive(config.keep_alive)
            .workers(config.start_workers);

        server.run().await.expect("Failed run HttpServer in Actix");

        Ok(())
    }
}
