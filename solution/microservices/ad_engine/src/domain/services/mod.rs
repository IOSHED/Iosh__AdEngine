//! # Services
//! Each service can manage multiple repositories, providing access to data from
//! different sources. Services usually contain business logic related to
//! changes or manipulations of the data they provide.

mod error;
mod redis_service;
mod user_service;

pub use error::ServiceError;
pub use redis_service::RedisService;
pub use user_service::UserService;

pub mod repository {
    pub use super::user_service::{IRegisterUser, IUserAreExists};
}

pub type ServiceResult<T> = Result<T, ServiceError>;
