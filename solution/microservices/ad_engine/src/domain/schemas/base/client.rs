#[derive(serde::Serialize, serde::Deserialize, validator::Validate, utoipa::ToSchema, Debug)]
pub struct ClientProfileSchema {
    #[schema(example = "3fa85f64-5717-4562-b3fc-2c963f66afa6", format = "uuid v4")]
    pub client_id: uuid::Uuid,

    #[schema(example = "my_login", max_length = 64, min_length = 2)]
    #[validate(length(min = 2, max = 64, message = "Login must be between 2 and 64 characters"))]
    pub login: String,

    #[schema(example = "Moscow, mcad", max_length = 128, min_length = 2)]
    #[validate(length(min = 2, max = 128, message = "Location must be between 2 and 64 characters"))]
    pub location: String,

    #[schema(example = "MALE")]
    #[validate(
        length(min = 4, max = 6, message = "Gender must be between 4 and 6 characters"),
        regex(
            path = "crate::domain::validators::RE_GENDER",
            message = "Gender not equal MALE or FEMALE"
        )
    )]
    pub gender: String,

    #[schema(example = 34, maximum = 160, minimum = 1)]
    #[validate(range(min = 1, max = 160, message = "Age not must be over 160"))]
    pub age: u8,
}
