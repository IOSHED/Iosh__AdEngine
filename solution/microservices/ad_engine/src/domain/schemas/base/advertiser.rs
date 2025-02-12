#[derive(serde::Serialize, serde::Deserialize, validator::Validate, utoipa::ToSchema, Debug)]
pub struct AdvertiserProfileSchema {
    #[schema(example = "3fa85f64-5717-4562-b3fc-2c963f66afa6", format = "uuid v4")]
    #[validate(length(equal = 36, message = "Len UUID v4 must be equal 32 (36)"))]
    pub advertiser_id: String,

    #[schema(example = "my_name", max_length = 64, min_length = 2)]
    #[validate(length(min = 2, max = 64, message = "Name must be between 2 and 64 characters"))]
    pub name: String,
}
