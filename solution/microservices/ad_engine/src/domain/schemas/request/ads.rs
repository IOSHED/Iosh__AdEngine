#[derive(serde::Deserialize, utoipa::ToSchema, validator::Validate, Debug)]
#[schema(title = "UUID client", description = "UUID client which clicked ads")]
pub struct AdClickRequest {
    #[schema(example = "3fa85f64-5717-4562-b3fc-2c963f66afa6", format = "uuid v4")]
    pub client_id: uuid::Uuid,
}
