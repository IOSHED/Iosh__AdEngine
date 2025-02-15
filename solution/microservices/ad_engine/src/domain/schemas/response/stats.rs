#[derive(Debug, serde::Serialize, utoipa::ToSchema, validator::Validate)]
#[schema(title = "Stats", description = "View stat")]
pub struct StatResponse {
    #[schema(example = 75)]
    pub impressions_count: u32,
    #[schema(example = 25)]
    pub clicks_count: u32,
    #[schema(example = 33.3)]
    pub conversion: f64,
    #[schema(example = 5550.0)]
    pub spent_impressions: f64,
    #[schema(example = 3550.0)]
    pub spent_clicks: f64,
    #[schema(example = 9100.0)]
    pub spent_total: f64,
}

#[derive(Debug, Default, Clone, serde::Serialize, utoipa::ToSchema, validator::Validate)]
#[schema(title = "Stat by day", description = "View stats by day")]
pub struct StatDailyResponse {
    #[schema(example = 75)]
    pub impressions_count: u32,
    #[schema(example = 25)]
    pub clicks_count: u32,
    #[schema(example = 33.3)]
    pub conversion: f64,
    #[schema(example = 5550.0)]
    pub spent_impressions: f64,
    #[schema(example = 3550.0)]
    pub spent_clicks: f64,
    #[schema(example = 9100.0)]
    pub spent_total: f64,
    #[schema(example = 1)]
    pub date: u32,
}
