mod exception;
mod http_client;
pub mod routers;
pub use http_client::HttpServer;

use crate::domain;

pub type ActixResult<T> = Result<T, domain::services::ServiceError>;
