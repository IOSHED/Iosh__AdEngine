#[derive(Debug, serde::Serialize, utoipa::ToSchema, validator::Validate)]
#[schema(
    title = "Time view global settings",
    description = "View global settings for time advance"
)]
pub struct TimeAdvanceResponse {
    #[schema(example = 1)]
    pub current_date: usize,
}
