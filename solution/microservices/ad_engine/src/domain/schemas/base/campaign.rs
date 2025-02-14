#[derive(serde::Serialize, serde::Deserialize, validator::Validate, utoipa::ToSchema, Debug, Clone, PartialEq)]
pub struct CampaignSchema {
    #[schema(example = "3fa85f64-5717-4562-b3fc-2c963f66afa6", format = "uuid v4")]
    pub campaign_id: uuid::Uuid,
    #[schema(example = "3fa85f64-5717-4562-b3fc-2c963f66afa6", format = "uuid v4")]
    pub advertiser_id: uuid::Uuid,

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
    pub gender: String,

    #[schema(example = 18, minimum = 0)]
    #[validate(range(min = 0, message = "age must be under or equal 0"))]
    pub age_from: u8,
    #[schema(example = 18, minimum = 0)]
    #[validate(range(min = 0, message = "age must be under or equal 0"))]
    pub age_to: u8,

    #[schema(example = "Moscow, mcad")]
    pub location: String,
}
