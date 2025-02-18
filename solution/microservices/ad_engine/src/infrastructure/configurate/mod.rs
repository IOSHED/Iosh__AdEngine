//! # Configuration
//! The project is configured from the [/conf](/solution/conf/) and `env`.

//! It is recommended to explicitly use the environment variable
//! `APP_ENVIRONMENT` (values: `prod`, `local`), otherwise the default value
//! (`prod`) will be substituted. For example:

//! ```python
//! APP_ENVIRONMENT="local"
//! ```

//! The configuration consists of three consecutive steps:

//! 1. **Load** configuration from [base.yaml](/solution/conf/base.yaml).
//! 2. **Load or rewrite** configuration from
//!    [local.yaml](/solution/conf/local.yaml) or
//!    [prod.yaml](/solution/conf/prod.yaml) file according to the variable
//!    `APP_ENVIRONMENT`.
//! 3. **Load or rewrite** confguration from `ENV` variable or
//!    [.env](/solution/.env) file. In this case, the nesting is determined by
//!    the separator `__`. You should definitely add a prefix `APP__` to the
//!    variable. For example:

//!    ```python
//!    APP_ENVIRONMENT="local"

//!    APP__HTTP_SERVER__PORT=8000  # http_server.port
//!    APP__HTTP_SERVER__HOST="127.0.0.1"   # http_server.host
//!    ```

mod deserializer;
mod schemas;

pub use schemas::{Config, CorsConfig, HttpServerConfig, LoggerConfig, PostgresConfig, RedisConfig};

pub async fn parse(path_to_conf_file: std::path::PathBuf) -> Config {
    dotenv::dotenv().ok();

    let environment = std::env::var("APP_ENVIRONMENT").unwrap_or("prod".into());
    if !["local", "prod"].contains(&environment.as_str()) {
        panic!("Failed load APP_ENVIRONMENT. APP_ENVIRONMENT must be `local` or `prod`.");
    }

    let parsing_shemas: schemas::Config = config::Config::builder()
        .add_source(config::File::from(path_to_conf_file.join("base")).required(true))
        .add_source(config::File::from(path_to_conf_file.join(environment)).required(true))
        .add_source(config::Environment::with_prefix("APP").separator("__"))
        .build()
        .expect("Filed set source for config")
        .try_deserialize()
        .expect("Failed parsing config");

    parsing_shemas
}
