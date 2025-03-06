#[derive(Debug, serde::Deserialize, utoipa::ToSchema, validator::Validate)]
#[schema(
    title = "Time Advance Configuration",
    description = "Configuration parameters for advancing the global system time",
    example = json!({
        "current_date": 1
    })
)]
/// Represents a request to advance time in the system
///
/// This struct is used to control global time settings and advance the current
/// date. It ensures the date value is non-negative through validation.
pub struct TimeAdvanceRequest {
    /// The date value to advance to
    ///
    /// Must be a non-negative integer representing the target date.
    /// The system will advance time to this specified date.
    #[schema(example = 1, minimum = 0)]
    #[validate(range(min = 0, message = "Date value must be non-negative"))]
    pub current_date: u32,
}
