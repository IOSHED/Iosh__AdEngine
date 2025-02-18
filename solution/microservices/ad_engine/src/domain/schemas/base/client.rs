
#[derive(serde::Serialize, serde::Deserialize, validator::Validate, utoipa::ToSchema, Debug, Clone, PartialEq)]
#[schema(
    example = json!({
        "client_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
        "login": "my_login", 
        "location": "Moscow, mcad",
        "gender": "MALE",
        "age": 34
    }),
    title = "Client Profile",
    description = "Schema containing core client profile information including identification, demographics and location",
)]
pub struct ClientProfileSchema {
    /// Unique identifier for the client
    #[schema(example = "3fa85f64-5717-4562-b3fc-2c963f66afa6", format = "uuid v4")]
    pub client_id: uuid::Uuid,

    /// Client's login username
    #[schema(example = "my_login")]
    pub login: String,

    /// Client's geographical location
    #[schema(example = "Moscow, mcad")]
    pub location: String,

    /// Client's gender - must be either 'MALE' or 'FEMALE'
    #[schema(example = "MALE")]
    #[validate(
        length(min = 4, max = 6, message = "Gender must be between 4 and 6 characters"),
        regex(
            path = "crate::domain::validators::RE_GENDER",
            message = "Gender not equal MALE or FEMALE"
        )
    )]
    pub gender: String,

    /// Client's age in years
    #[schema(
        example = 34,
        maximum = 160,
        minimum = 1
    )]
    #[validate(range(min = 1, max = 160, message = "Age not must be over 160"))]
    pub age: u8,
}
