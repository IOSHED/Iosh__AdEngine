#[derive(serde::Deserialize, validator::Validate, utoipa::ToSchema, Debug)]
#[schema(title = "User are exists Request", description = "Info for finding user")]
pub struct UserAreExistRequest {
    #[schema(example = "123456789")]
    #[validate(range(min = 1, message = "Telegram ID must be a positive integer"))]
    pub telegram_id: usize,
}
