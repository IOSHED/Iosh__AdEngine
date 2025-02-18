#[derive(Debug, serde::Serialize, utoipa::ToSchema, validator::Validate)]
#[schema(
    title = "Campaign Statistics",
    description = "Aggregated statistics for campaign performance including impressions, clicks and spend"
)]
/// Represents a statistics response containing impression and click metrics
pub struct StatResponse {
    /// Total number of ad impressions served
    #[schema(example = 75)]
    pub impressions_count: u32,

    /// Total number of clicks received
    #[schema(example = 25)]
    pub clicks_count: u32,

    /// Click-through rate (CTR) as percentage of impressions that resulted in
    /// clicks
    #[schema(example = 33.3)]
    pub conversion: f64,

    /// Total cost spent on impressions in campaign currency
    #[schema(example = 5550.0)]
    pub spent_impressions: f64,

    /// Total cost spent on clicks in campaign currency
    #[schema(example = 3550.0)]
    pub spent_clicks: f64,

    /// Total campaign spend (impressions + clicks) in campaign currency
    #[schema(example = 9100.0)]
    pub spent_total: f64,
}

#[derive(Debug, Default, Clone, serde::Serialize, utoipa::ToSchema, validator::Validate)]
#[schema(
    title = "Daily Campaign Statistics",
    description = "Daily breakdown of campaign performance metrics including impressions, clicks and spend"
)]
/// Represents daily statistics for campaign performance metrics
pub struct StatDailyResponse {
    /// Number of ad impressions served on this day
    #[schema(example = 75)]
    pub impressions_count: u32,

    /// Number of clicks received on this day
    #[schema(example = 25)]
    pub clicks_count: u32,

    /// Daily click-through rate (CTR) as percentage
    #[schema(example = 33.3)]
    pub conversion: f64,

    /// Cost of impressions for this day in campaign currency
    #[schema(example = 5550.0)]
    pub spent_impressions: f64,

    /// Cost of clicks for this day in campaign currency
    #[schema(example = 3550.0)]
    pub spent_clicks: f64,

    /// Total spend for this day in campaign currency
    #[schema(example = 9100.0)]
    pub spent_total: f64,

    /// Advanced time
    #[schema(example = 1)]
    pub date: u32,
}
