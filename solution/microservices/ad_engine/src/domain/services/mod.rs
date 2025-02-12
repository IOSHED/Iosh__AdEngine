//! # Services
//! Each service can manage multiple repositories, providing access to data from
//! different sources. Services usually contain business logic related to
//! changes or manipulations of the data they provide.

mod advertiser_service;
mod client_service;
mod error;
mod ml_score_service;
mod redis_service;

pub use advertiser_service::AdvertiserService;
pub use client_service::ClientService;
pub use error::ServiceError;
pub use ml_score_service::MlScoreService;
pub use redis_service::RedisService;

pub mod repository {
    pub use super::{
        advertiser_service::{IGetAdvertiserById, IRegisterBulkAdvertiser},
        client_service::{IGetClientById, IRegisterBulkClient},
        ml_score_service::ISetMlScore,
    };
}

pub type ServiceResult<T> = Result<T, ServiceError>;
