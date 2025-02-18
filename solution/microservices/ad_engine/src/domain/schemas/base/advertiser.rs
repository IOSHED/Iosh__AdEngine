#[derive(serde::Serialize, serde::Deserialize, validator::Validate, utoipa::ToSchema, Debug, Clone, PartialEq)]
#[schema(
    title = "Represents the profile data for an advertiser",
    description = "This schema defines the core attributes that make up an advertiser's profile",
    example = json!({
        "advertiser_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
        "name": "my_name"
    })
)]
/// Represents the profile data for an advertiser
///
/// This schema defines the core attributes that make up an advertiser's profile
pub struct AdvertiserProfileSchema {
    /// Unique identifier for the advertiser
    ///
    /// This UUID v4 is used as the primary key to identify advertisers across
    /// the system
    #[schema(example = "3fa85f64-5717-4562-b3fc-2c963f66afa6", format = "uuid v4")]
    pub advertiser_id: uuid::Uuid,

    /// Display name of the advertiser
    ///
    /// This is the human-readable name used to identify the advertiser in the
    /// UI and reports
    #[schema(example = "my_name")]
    pub name: String,
}
