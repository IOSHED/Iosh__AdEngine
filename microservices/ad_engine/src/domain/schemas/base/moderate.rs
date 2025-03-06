/// Represents the global moderation settings configuration.
/// This struct is used to enable or disable moderation features system-wide.
#[derive(Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema, validator::Validate)]
#[schema(
    title = "Moderation Global Settings",
    description = "Configuration for enabling/disabling global moderation features",
    example = json!({
        "is_activate": true
    })
)]
pub struct ModerateSchema {
    /// Flag to enable/disable moderation features
    /// When true, moderation will be active across the system
    /// When false, moderation will be disabled
    #[schema(example = true)]
    pub is_activate: bool,
}
