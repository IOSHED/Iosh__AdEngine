mod ads;
mod compaign;
mod ml_score;
mod time;
pub use ads::AdClickRequest;
pub use compaign::{CampaignsCreateRequest, CampaignsGenerateTextRequest, CampaignsUpdateRequest};
pub use ml_score::MlScoreRequest;
pub use time::TimeAdvanceRequest;
