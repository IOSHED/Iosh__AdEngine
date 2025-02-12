#[derive(serde::Deserialize)]
pub struct Config {
    pub http_server: HttpServerConfig,
    pub logger: LoggerConfig,
    pub database: DatabaseConfig,
    pub cors: CorsConfig,
}

#[serde_with::serde_as]
#[derive(Clone, serde::Deserialize)]
pub struct HttpServerConfig {
    pub port: u16,
    pub host: std::net::Ipv4Addr,
    pub path_swagger_docs: std::path::PathBuf,
    pub path_openapi_docs: std::path::PathBuf,
    pub start_workers: usize,
    #[serde_as(as = "serde_with::DurationSeconds")]
    pub timeout_shutdown_workers: std::time::Duration,
    #[serde_as(as = "serde_with::DurationSeconds")]
    pub keep_alive: std::time::Duration,
    pub limit_size_json: usize,
}

#[derive(Clone, serde::Deserialize)]
pub struct LoggerConfig {
    #[serde(deserialize_with = "crate::infrastructure::configurate::deserializer::deserialize_level_filter")]
    pub max_level_cmd: tracing::level_filters::LevelFilter,
    #[serde(deserialize_with = "crate::infrastructure::configurate::deserializer::deserialize_level_filter")]
    pub max_level_file: tracing::level_filters::LevelFilter,
    #[serde(deserialize_with = "crate::infrastructure::configurate::deserializer::deserialize_level_filter")]
    pub max_level_error_file: tracing::level_filters::LevelFilter,
    pub log_dir: std::path::PathBuf,
}

#[derive(Clone, serde::Deserialize)]
pub struct DatabaseConfig {
    pub postgres: PostgresConfig,
    pub redis: RedisConfig,
}

#[derive(Clone, serde::Deserialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: usize,
    pub db: usize,
}

#[derive(Clone, serde::Deserialize)]
pub struct PostgresConfig {
    pub max_connections: u32,
    pub postgres_conn: String,
}

#[derive(Clone, serde::Deserialize)]
pub struct CorsConfig {
    pub allowed_origin: String,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
    pub max_age: usize,
}
