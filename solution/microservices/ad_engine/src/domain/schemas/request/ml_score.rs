#[derive(serde::Deserialize, utoipa::ToSchema, validator::Validate, Debug, Clone, PartialEq)]
#[schema(
    title = "ML Score Binding Request",
    description = "Request payload for binding a machine learning score between a client and advertiser"
)]
/// Represents a request to bind an ML score between a client and advertiser
///
/// This struct is used to associate a machine learning score with a specific
/// client-advertiser pair in the system.
pub struct MlScoreRequest {
    /// Unique identifier for the client
    ///
    /// Must be a valid UUID v4
    #[schema(example = "3fa85f64-5717-4562-b3fc-2c963f66afa6", format = "uuid v4")]
    pub client_id: uuid::Uuid,

    /// Unique identifier for the advertiser
    ///
    /// Must be a valid UUID v4
    #[schema(example = "3fa85f64-8717-4562-b3fc-2c963f66afa6", format = "uuid v4")]
    pub advertiser_id: uuid::Uuid,

    /// Machine learning score value
    ///
    /// A floating point value representing the ML model's score
    #[schema(example = 0.4, minimum = 0.0)]
    pub score: f64,
}
