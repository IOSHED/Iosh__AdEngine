#[derive(serde::Serialize, serde::Deserialize, validator::Validate, utoipa::ToSchema, Debug, Clone, PartialEq)]
#[schema(
    title = "Campaign configuration schema",
    description = "Represents the configuration for an advertising campaign including targeting parameters",
    example = json!({
        "campaign_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
        "advertiser_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6", 
        "impressions_limit": 25,
        "clicks_limit": 105,
        "cost_per_impressions": 100.0,
        "cost_per_clicks": 150.0,
        "ad_title": "Mega Ad",
        "ad_text": "His omega must be Ad",
        "start_date": 3,
        "end_date": 5,
        "targeting": {
            "gender": "MALE",
            "age_from": 18,
            "age_to": 18,
            "location": "Moscow, mcad"
        }
    })
)]
/// Campaign configuration schema
///
/// Represents the configuration for an advertising campaign including targeting
/// parameters
pub struct CampaignSchema {
    /// Unique identifier for the campaign
    #[schema(example = "3fa85f64-5717-4562-b3fc-2c963f66afa6", format = "uuid v4")]
    pub campaign_id: uuid::Uuid,

    /// Unique identifier for the advertiser
    #[schema(example = "3fa85f64-5717-4562-b3fc-2c963f66afa6", format = "uuid v4")]
    pub advertiser_id: uuid::Uuid,

    /// Maximum number of impressions allowed for this campaign
    #[schema(example = 205, minimum = 0)]
    pub impressions_limit: u32,

    /// Maximum number of clicks allowed for this campaign
    #[schema(example = 105, minimum = 0)]
    pub clicks_limit: u32,

    /// Cost per thousand impressions (CPM) in campaign currency
    #[schema(example = 100.0, minimum = 0)]
    pub cost_per_impressions: f64,

    /// Cost per click (CPC) in campaign currency
    #[schema(example = 150.0, minimum = 0)]
    pub cost_per_clicks: f64,

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
    pub targeting: TargetingCampaignSchema,
}

#[derive(
    serde::Serialize, serde::Deserialize, validator::Validate, utoipa::ToSchema, Debug, Default, Clone, PartialEq,
)]
#[schema(
    title = "Campaign targeting configuration",
    description = "Defines the demographic and geographic targeting parameters for a campaign",
    example = json!({
        "gender": "MALE",
        "age_from": 18,
        "age_to": 18,
        "location": "Moscow, mcad"
    })
)]
/// Campaign targeting configuration
///
/// Defines the demographic and geographic targeting parameters for a campaign
pub struct TargetingCampaignSchema {
    /// Target gender (MALE or FEMALE)
    #[schema(example = "MALE")]
    #[validate(
        length(min = 4, max = 6, message = "Gender must be between 4 and 6 characters"),
        regex(
            path = "crate::domain::validators::RE_GENDER",
            message = "Gender not equal MALE or FEMALE"
        )
    )]
    pub gender: Option<String>,

    /// Minimum age for targeting
    #[schema(example = 18, minimum = 0)]
    #[validate(range(min = 0, message = "age must be under or equal 0"))]
    pub age_from: Option<u8>,

    /// Maximum age for targeting
    #[schema(example = 18, minimum = 0)]
    #[validate(range(min = 0, message = "age must be under or equal 0"))]
    pub age_to: Option<u8>,

    /// Geographic location for targeting
    #[schema(example = "Moscow, mcad")]
    pub location: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
/// Active campaign state
///
/// Represents the current state of an active campaign including performance
/// metrics
pub struct ActiveCampaignSchema {
    /// Unique identifier for the campaign
    pub campaign_id: uuid::Uuid,

    /// Unique identifier for the advertiser
    pub advertiser_id: uuid::Uuid,

    /// Maximum number of impressions allowed
    pub impressions_limit: u32,

    /// Maximum number of clicks allowed
    pub clicks_limit: u32,

    /// Cost per thousand impressions (CPM)
    pub cost_per_impressions: f64,

    /// Cost per click (CPC)
    pub cost_per_clicks: f64,

    /// Advertisement title
    pub ad_title: String,

    /// Advertisement text content
    pub ad_text: String,

    /// Campaign start date
    pub start_date: u32,

    /// Campaign end date
    pub end_date: u32,

    /// List of client IDs who viewed the ad
    pub view_clients_id: Vec<uuid::Uuid>,

    /// List of client IDs who clicked the ad
    pub click_clients_id: Vec<uuid::Uuid>,

    /// Campaign targeting parameters
    pub targeting: TargetingCampaignSchema,
}

impl std::convert::From<(CampaignSchema, Vec<uuid::Uuid>, Vec<uuid::Uuid>)> for ActiveCampaignSchema {
    fn from(data: (CampaignSchema, Vec<uuid::Uuid>, Vec<uuid::Uuid>)) -> Self {
        let campaign = data.0;

        Self {
            campaign_id: campaign.campaign_id,
            advertiser_id: campaign.advertiser_id,
            impressions_limit: campaign.impressions_limit,
            clicks_limit: campaign.clicks_limit,
            cost_per_impressions: campaign.cost_per_impressions,
            cost_per_clicks: campaign.cost_per_clicks,
            ad_title: campaign.ad_title,
            ad_text: campaign.ad_text,
            start_date: campaign.start_date,
            end_date: campaign.end_date,
            view_clients_id: data.1,
            click_clients_id: data.2,
            targeting: campaign.targeting,
        }
    }
}
