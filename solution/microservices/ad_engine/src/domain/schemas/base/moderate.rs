#[derive(Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema, validator::Validate)]
#[schema(title = "Moderate set global settings", description = "Set moderate settings")]
pub struct ModerateSchema {
    #[schema(example = true)]
    pub is_activate: bool,
}
