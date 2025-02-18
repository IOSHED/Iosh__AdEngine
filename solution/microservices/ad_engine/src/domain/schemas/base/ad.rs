#[derive(serde::Serialize, serde::Deserialize, validator::Validate, utoipa::ToSchema, Debug, Clone, PartialEq)]
#[schema(
    title = "Advertisement",
    description = "Schema for an advertisement containing its ID, title, content and advertiser information",
    example = json!({
        "ad_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
        "ad_title": "Mega Ad", 
        "ad_text": "His omega must be Ad",
        "advertiser_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6"
    })
)]
/// Represents an advertisement schema
///
/// This struct contains all the necessary information for an advertisement
/// including its unique identifier, title, content text and the ID of the
/// advertiser who created it.
pub struct AdSchema {
    /// Unique identifier for the advertisement
    #[schema(example = "3fa85f64-5717-4562-b3fc-2c963f66afa6", format = "uuid v4")]
    pub ad_id: uuid::Uuid,

    /// Title of the advertisement
    #[schema(example = "Mega Ad")]
    pub ad_title: String,

    /// Main content text of the advertisement
    #[schema(example = "His omega must be Ad")]
    pub ad_text: String,

    /// Unique identifier of the advertiser who created this ad
    #[schema(example = "3fa85f64-5717-4562-b3fc-2c963f66afa6", format = "uuid v4")]
    pub advertiser_id: uuid::Uuid,
}
