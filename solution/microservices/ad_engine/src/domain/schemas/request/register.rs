#[derive(serde::Deserialize, validator::Validate, utoipa::ToSchema, Debug)]
#[schema(
    title = "User Registration Request",
    description = "Payload for registering new user"
)]
pub struct RegisterRequest {
    #[schema(example = "123456789")]
    #[validate(range(min = 1, message = "Telegram ID must be a positive integer"))]
    pub telegram_id: usize,

    #[schema(example = "1995-05-15", pattern = r"^\d{4}-\d{2}-\d{2}$", format = "ISO 8601")]
    #[validate(
        length(
            max = 10,
            message = "Birth day must be in the format YYYY-MM-DD and up to 10 characters long"
        ),
        regex(
            path = "crate::domain::validators::RE_DATE",
            message = "Birth day must be a valid date in YYYY-MM-DD format"
        )
    )]
    pub birth_day: String,

    #[schema(example = "New York", max_length = 100)]
    #[validate(length(max = 100, message = "City name must be under 100 characters"))]
    pub city: Option<String>,

    #[schema(example = "RU", max_length = 2, min_length = 2, pattern = r"[a-z-A-Z]{2}")]
    #[validate(
        length(equal = 2, message = "Len country codes should be equal 2"),
        regex(path = "crate::domain::validators::RE_ALPHA2", message = "Country code is not valid")
    )]
    pub country_code: Option<String>,

    #[schema(example = "[\"music\", \"sports\"]")]
    #[validate(length(max = 10, message = "Interests must have at most 10 items"))]
    pub interests: Vec<String>,

    #[schema(example = "I love coding")]
    #[validate(length(max = 255, message = "Bio must be under 255 characters"))]
    pub bio: Option<String>,

    #[schema(example = "37.7749")]
    #[validate(
        range(min = -90.0, max = 90.0, message = "Latitude must be between -90 and 90")
    )]
    pub latitude: f64,

    #[schema(example = "-122.4194")]
    #[validate(
        range(min = -180.0, max = 180.0, message = "Longitude must be between -180 and 180")
    )]
    pub longitude: f64,
}
