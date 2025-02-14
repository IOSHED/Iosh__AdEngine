#[derive(serde::Serialize, serde::Deserialize, validator::Validate, utoipa::ToSchema, Debug, Clone, PartialEq)]
pub struct AdSchema {
    #[schema(example = "3fa85f64-5717-4562-b3fc-2c963f66afa6", format = "uuid v4")]
    pub ad_id: uuid::Uuid,

    #[schema(example = "Mega Ad")]
    pub ad_title: String,

    #[schema(example = "His omega must be Ad")]
    pub ad_text: String,

    #[schema(example = "3fa85f64-5717-4562-b3fc-2c963f66afa6", format = "uuid v4")]
    pub advertiser_id: uuid::Uuid,
}
