use crate::domain;

#[derive(serde::Serialize, serde::Deserialize, validator::Validate, utoipa::ToSchema, Debug, Clone, PartialEq)]
#[schema(
    title = "Create Campaign Request",
    description = "Request payload for creating a new advertising campaign",
    example = json!({
        "impressions_limit": 105,
        "clicks_limit": 25,
        "cost_per_impression": 100.0,
        "cost_per_click": 150.0,
        "ad_title": "Mega Ad",
        "ad_text": "His omega must be Ad",
        "start_date": 3,
        "end_date": 5,
        "targeting": {}
    })
)]
/// Represents a request to create a new advertising campaign
pub struct CampaignsCreateRequest {
    /// Maximum number of impressions allowed for this campaign
    #[schema(example = 105, minimum = 0)]
    pub impressions_limit: u32,
    /// Maximum number of clicks allowed for this campaign
    #[schema(example = 205, minimum = 0)]
    pub clicks_limit: u32,

    /// Cost per thousand impressions (CPM)
    #[schema(example = 100.0, minimum = 0)]
    pub cost_per_impression: f64,
    /// Cost per click (CPC)
    #[schema(example = 150.0, minimum = 0)]
    pub cost_per_click: f64,

    /// Title of the advertisement
    #[schema(example = "Mega Ad")]
    pub ad_title: String,
    /// Main text content of the advertisement
    #[schema(example = "His omega must be Ad")]
    pub ad_text: String,

    /// Campaign start date (Unix timestamp)
    #[schema(example = 3)]
    #[validate(range(min = 0, message = "date must be under or equal 0"))]
    pub start_date: u32,

    /// Campaign end date (Unix timestamp)
    #[schema(example = 5)]
    #[validate(range(min = 0, message = "date must be under or equal 0"))]
    pub end_date: u32,

    /// Targeting criteria for the campaign
    pub targeting: domain::schemas::TargetingCampaignSchema,
}

#[derive(serde::Serialize, serde::Deserialize, validator::Validate, utoipa::ToSchema, Debug, Clone, PartialEq)]
#[schema(
    title = "Update Campaign Request",
    description = "Request payload for updating an existing advertising campaign",
    example = json!({
        "impressions_limit": 105,
        "clicks_limit": 205,
        "cost_per_impression": 100.0,
        "cost_per_click": 150.0,
        "ad_title": "Mega Ad",
        "ad_text": "His omega must be Ad",
        "start_date": 3,
        "end_date": 5,
        "targeting": {}
    })
)]

/// Represents a request to update an existing advertising campaign
pub struct CampaignsUpdateRequest {
    /// Maximum number of impressions allowed for this campaign
    #[schema(example = 105, minimum = 0)]
    pub impressions_limit: u32,
    /// Maximum number of clicks allowed for this campaign
    #[schema(example = 205, minimum = 0)]
    pub clicks_limit: u32,

    /// Cost per thousand impressions (CPM)
    #[schema(example = 100.0, minimum = 0)]
    pub cost_per_impression: f64,
    /// Cost per click (CPC)
    #[schema(example = 150.0, minimum = 0)]
    pub cost_per_click: f64,

    /// Title of the advertisement
    #[schema(example = "Mega Ad")]
    pub ad_title: String,
    /// Main text content of the advertisement
    #[schema(example = "His omega must be Ad")]
    pub ad_text: String,

    /// Campaign start date (Unix timestamp)
    #[schema(example = 3)]
    #[validate(range(min = 0, message = "date must be under or equal 0"))]
    pub start_date: u32,

    /// Campaign end date (Unix timestamp)
    #[schema(example = 5)]
    #[validate(range(min = 0, message = "date must be under or equal 0"))]
    pub end_date: u32,

    /// Targeting criteria for the campaign
    pub targeting: domain::schemas::TargetingCampaignSchema,
}

#[derive(serde::Deserialize, validator::Validate, utoipa::ToSchema, Debug)]
#[schema(
    title = "Generate Ad Text Request",
    description = "Request payload for generating advertisement text content",
    example = json!({
        "generate_type": "ALL",
        "ad_title": "Mega Ad",
        "ad_text": "His omega must be Ad"
    })
)]

/// Represents a request to generate ad text content
pub struct CampaignsGenerateTextRequest {
    /// Type of text generation (TITLE, TEXT, or ALL)
    #[schema(example = "ALL")]
    #[validate(regex(
        path = "crate::domain::validators::RE_GENERATE_TYPE",
        message = "Generate type not equal TITLE or TEXT or ALL"
    ))]
    pub generate_type: String,
    /// Optional existing ad title to base generation on
    #[schema(example = "Mega Ad")]
    pub ad_title: Option<String>,
    /// Optional existing ad text to base generation on
    #[schema(example = "His omega must be Ad")]
    pub ad_text: Option<String>,
}

impl std::convert::From<domain::schemas::CampaignSchema> for CampaignsUpdateRequest {
    fn from(campaign: domain::schemas::CampaignSchema) -> Self {
        Self {
            impressions_limit: campaign.impressions_limit,
            clicks_limit: campaign.clicks_limit,
            cost_per_impression: campaign.cost_per_impression,
            cost_per_click: campaign.cost_per_click,
            ad_title: campaign.ad_title,
            ad_text: campaign.ad_text,
            start_date: campaign.start_date,
            end_date: campaign.end_date,
            targeting: campaign.targeting,
        }
    }
}
