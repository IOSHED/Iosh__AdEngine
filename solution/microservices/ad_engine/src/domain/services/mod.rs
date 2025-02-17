//! # Services
//! Each service can manage multiple repositories, providing access to data from
//! different sources. Services usually contain business logic related to
//! changes or manipulations of the data they provide.

mod ads_service;
mod advertiser_service;
mod aggregate_stat_service;
mod campaign_image;
mod campaigns_service;
mod campaigns_stat_service;
mod client_service;
mod error;
mod ml_score_service;
mod moderate_text;
mod prometheus_service;
mod redis_service;
mod upload_image;
mod yandex_gpt_service;

pub use ads_service::AdsService;
pub use advertiser_service::AdvertiserService;
pub use aggregate_stat_service::AggregateStatService;
pub use campaign_image::CampaignImageService;
pub use campaigns_service::CampaignService;
pub use campaigns_stat_service::CampaignStatService;
pub use client_service::ClientService;
pub use error::ServiceError;
pub use ml_score_service::MlScoreService;
pub use moderate_text::ModerateTextService;
pub use prometheus_service::PrometheusService;
pub use redis_service::RedisService;
pub use upload_image::UploadImageService;
pub use yandex_gpt_service::YandexGptService;

pub mod repository {
    pub use super::{
        ads_service::IGetMlScores,
        advertiser_service::{IGetAdvertiserById, IRegisterBulkAdvertiser},
        campaign_image::{IDeleteCampaignImage, IGetCampaignImage, IGetCampaignNamesImage},
        campaigns_service::{
            ICreateCampaign, IDeleteCampaign, IGetActiveCampaignList, IGetCampaignById, IGetCampaignList,
            IGetIdsCampaign, ISearchCampaign, IUpdateCampaign,
        },
        campaigns_stat_service::{IClickCampaign, IGetDailyStat, IGetOrCreateUniqIdForStatCampaign, IViewCampaign},
        client_service::{IGetClientById, IRegisterBulkClient},
        ml_score_service::ISetMlScore,
        moderate_text::IGetAbusiveWords,
        upload_image::IUploadCampaignImage,
    };
}

pub type ServiceResult<T> = Result<T, ServiceError>;
