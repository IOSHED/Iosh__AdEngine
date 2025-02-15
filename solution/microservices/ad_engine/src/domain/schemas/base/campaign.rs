#[derive(serde::Serialize, serde::Deserialize, validator::Validate, utoipa::ToSchema, Debug, Clone, PartialEq)]
pub struct CampaignSchema {
    #[schema(example = "3fa85f64-5717-4562-b3fc-2c963f66afa6", format = "uuid v4")]
    pub campaign_id: uuid::Uuid,
    #[schema(example = "3fa85f64-5717-4562-b3fc-2c963f66afa6", format = "uuid v4")]
    pub advertiser_id: uuid::Uuid,

    #[schema(example = 205, minimum = 0)]
    pub impressions_limit: u32,
    #[schema(example = 105, minimum = 0)]
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

    pub targeting: TargetingCampaignSchema,
}

#[derive(serde::Serialize, serde::Deserialize, validator::Validate, utoipa::ToSchema, Debug, Clone, PartialEq)]
pub struct TargetingCampaignSchema {
    #[schema(example = "MALE")]
    #[validate(
        length(min = 4, max = 6, message = "Gender must be between 4 and 6 characters"),
        regex(
            path = "crate::domain::validators::RE_GENDER",
            message = "Gender not equal MALE or FEMALE"
        )
    )]
    pub gender: Option<String>,

    #[schema(example = 18, minimum = 0)]
    #[validate(range(min = 0, message = "age must be under or equal 0"))]
    pub age_from: Option<u8>,
    #[schema(example = 18, minimum = 0)]
    #[validate(range(min = 0, message = "age must be under or equal 0"))]
    pub age_to: Option<u8>,

    #[schema(example = "Moscow, mcad")]
    pub location: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct ActiveCampaignSchema {
    pub campaign_id: uuid::Uuid,
    pub advertiser_id: uuid::Uuid,

    pub impressions_limit: u32,
    pub clicks_limit: u32,

    pub cost_per_impressions: f64,
    pub cost_per_clicks: f64,

    pub ad_title: String,
    pub ad_text: String,

    pub start_date: u32,
    pub end_date: u32,

    pub view_clients_id: Vec<uuid::Uuid>,
    pub click_clients_id: Vec<uuid::Uuid>,

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
