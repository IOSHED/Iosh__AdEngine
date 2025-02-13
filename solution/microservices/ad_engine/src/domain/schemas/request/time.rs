#[derive(Debug, serde::Deserialize, utoipa::ToSchema, validator::Validate)]
#[schema(
    title = "Time set global settings",
    description = "Set global settings for time advance"
)]
pub struct TimeAdvanceRequest {
    #[schema(example = 1)]
    #[validate(range(min = 0, message = "date must be under or equal 0"))]
    pub current_date: u32,
}
