use crate::domain;

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct RegisterResponse {
    pub profile: domain::schemas::UserProfileSchema,
}
