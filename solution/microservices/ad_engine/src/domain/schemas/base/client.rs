#[derive(serde::Serialize, serde::Deserialize, validator::Validate, utoipa::ToSchema, Debug, Clone, PartialEq)]
pub struct ClientProfileSchema {
    #[schema(example = "3fa85f64-5717-4562-b3fc-2c963f66afa6", format = "uuid v4")]
    pub client_id: uuid::Uuid,

    #[schema(example = "my_login")]
    pub login: String,

    #[schema(example = "Moscow, mcad")]
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
