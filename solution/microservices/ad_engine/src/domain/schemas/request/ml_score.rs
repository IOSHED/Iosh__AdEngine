#[derive(Debug, serde::Deserialize, utoipa::ToSchema, validator::Validate)]
#[schema(
    title = "Bind score to client and advertiser",
    description = "Bind ML score to client and advertiser"
)]
pub struct MlScoreRequest {
    #[schema(example = "3fa85f64-5717-4562-b3fc-2c963f66afa6")]
    pub client_id: String,
    #[schema(example = "3fa85f64-8717-4562-b3fc-2c963f66afa6")]
    pub advertiser_id: String,
    #[schema(example = 0.4)]
    pub score: f64,
}
