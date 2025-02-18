/// Represents a request for tracking ad clicks from a client
/// Contains the unique identifier of the client who clicked the ad
#[derive(serde::Deserialize, utoipa::ToSchema, validator::Validate, Debug)]
#[schema(
    title = "Ad Click Request",
    description = "Request payload for tracking ad clicks from clients. Used to record when a specific client interacts with an advertisement.",
    example = json!({
        "client_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6"
    })
)]
pub struct AdClickRequest {
    /// Unique identifier for the client
    /// Must be a valid UUID v4 format
    #[schema(example = "3fa85f64-5717-4562-b3fc-2c963f66afa6", format = "uuid")]
    pub client_id: uuid::Uuid,
}
