#[derive(Debug, serde::Serialize, utoipa::ToSchema, validator::Validate)]
#[schema(
    title = "Time Advance Settings Response",
    description = "Response object containing the current system date and time advance configuration"
)]
/// Represents the response structure for time advance settings
/// Contains information about the current date in the system
pub struct TimeAdvanceResponse {
    /// The current date value in the system
    #[schema(example = 1, minimum = 0)]
    pub current_date: u32,
}
