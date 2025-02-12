//! # Services
//! Each service can manage multiple repositories, providing access to data from
//! different sources. Services usually contain business logic related to
//! changes or manipulations of the data they provide.

mod client_service;
mod error;
mod redis_service;

pub use client_service::ClientService;
pub use error::ServiceError;
pub use redis_service::RedisService;

pub mod repository {
    pub use super::client_service::{IGetClientById, IRegisterBulkClient};
}

pub type ServiceResult<T> = Result<T, ServiceError>;
