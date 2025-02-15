use crate::domain;

#[derive(serde::Serialize, serde::Deserialize, validator::Validate, utoipa::ToSchema, Debug, Clone, PartialEq)]
pub struct CampaignsCreateRequest {
    #[schema(example = 105, minimum = 0)]
    pub impressions_limit: u32,
    #[schema(example = 205, minimum = 0)]
    pub clicks_limit: u32,

    #[schema(example = 100.0, minimum = 0)]
    pub cost_per_impressions: f64,
    #[schema(example = 150.0, minimum = 0)]
    pub cost_per_clicks: f64,

    #[schema(example = "Mega Ad")]
    pub ad_title: String,
    #[schema(example = "His omega must be Ad")]
    pub ad_text: String,

    #[schema(example = 3)]
    #[validate(range(min = 0, message = "date must be under or equal 0"))]
    pub start_date: u32,

    #[schema(example = 5)]
    #[validate(range(min = 0, message = "date must be under or equal 0"))]
    pub end_date: u32,

    pub targeting: domain::schemas::TargetingCampaignSchema,
}

#[derive(serde::Serialize, serde::Deserialize, validator::Validate, utoipa::ToSchema, Debug, Clone, PartialEq)]
pub struct CampaignsUpdateRequest {
    #[schema(example = 105, minimum = 0)]
    pub impressions_limit: u32,
    #[schema(example = 205, minimum = 0)]
    pub clicks_limit: u32,

    #[schema(example = 100.0, minimum = 0)]
    pub cost_per_impressions: f64,
    #[schema(example = 150.0, minimum = 0)]
    pub cost_per_clicks: f64,

    #[schema(example = "Mega Ad")]
    pub ad_title: String,
    #[schema(example = "His omega must be Ad")]
    pub ad_text: String,

    #[schema(example = 3)]
    #[validate(range(min = 0, message = "date must be under or equal 0"))]
    pub start_date: u32,

    #[schema(example = 5)]
    #[validate(range(min = 0, message = "date must be under or equal 0"))]
    pub end_date: u32,

    pub targeting: domain::schemas::TargetingCampaignSchema,
}
