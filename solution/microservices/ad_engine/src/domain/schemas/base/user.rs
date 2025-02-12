#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct UserProfileSchema {
    #[schema(example = 23421312)]
    pub telegram_id: usize,

    #[schema(example = "2007-04-17")]
    pub birth_day: String,

    #[schema(example = "Москва")]
    pub city: String,

    #[schema(example = "RU")]
    pub country_code: String,

    #[schema(example = "[\"Литература\", \"Спорт\"]")]
    pub interests: Vec<String>,

    #[schema(example = "Я люблю читать Шекспира")]
    pub bio: Option<String>,
}
