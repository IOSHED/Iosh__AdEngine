//! # Usecase
//! Use case performs a specific business operation and can interact with
//! multiple services. It manages the logic that defines how data and operations
//! are related to each other.

mod ads_click;
mod ads_get;
mod advertiser_bulk_register;
mod advertiser_profile;
mod campaigns_generator_text_usecase;
mod campaings_create;
mod campaings_delete;
mod campaings_get_by_id;
mod campaings_gete_list;
mod campaings_update;
mod client_bulk_register;
mod client_profile;
mod ml_score;
mod stat_campaign;
mod time_advance;
pub use ads_click::AdsClickUsecase;
pub use ads_get::AdsGetUsecase;
pub use advertiser_bulk_register::AdvertiserBulkRegisterUsecase;
pub use advertiser_profile::AdvertiserProfileUsecase;
pub use campaigns_generator_text_usecase::CampaignsGeneratorTextUsecase;
pub use campaings_create::CampaignsCreateUsecase;
pub use campaings_delete::CampaignsDeleteUsecase;
pub use campaings_get_by_id::CampaignsGetByIdUsecase;
pub use campaings_gete_list::CampaignsGetListUsecase;
pub use campaings_update::CampaignsUpdateUsecase;
pub use client_bulk_register::ClientBulkRegisterUsecase;
pub use client_profile::ClientProfileUsecase;
pub use ml_score::MlScoreUsecase;
pub use stat_campaign::StatCampaignUsecase;
pub use time_advance::TimeAdvanceUsecase;
