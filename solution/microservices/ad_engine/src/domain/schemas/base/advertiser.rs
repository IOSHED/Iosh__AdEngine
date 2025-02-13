#[derive(serde::Serialize, serde::Deserialize, validator::Validate, utoipa::ToSchema, Debug)]
pub struct AdvertiserProfileSchema {
    #[schema(example = "3fa85f64-5717-4562-b3fc-2c963f66afa6", format = "uuid v4")]
    pub advertiser_id: uuid::Uuid,

    #[schema(example = "my_name")]
    pub name: String,
}
