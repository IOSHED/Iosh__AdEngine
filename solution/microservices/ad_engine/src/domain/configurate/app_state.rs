use crate::infrastructure;

/// Application state containing configuration parameters for Yandex GPT
/// integration and ad recommendation weights
///
/// # Fields
/// * `yandex_api_key` - Authentication key for Yandex API access
/// * `yandex_folder_id` - Identifier for the Yandex resource folder
/// * `system_prompt_for_generate_title` - System prompt template for title
///   generation
/// * `system_prompt_for_generate_body` - System prompt template for body text
///   generation
/// * `ads_weight_profit` - Weight coefficient for profit in ad recommendations
///   (0.0 to 1.0)
/// * `ads_weight_relevance` - Weight coefficient for relevance in ad
///   recommendations (0.0 to 1.0)
/// * `ads_weight_fulfillment` - Weight coefficient for fulfillment in ad
///   recommendations (0.0 to 1.0)
/// * `ads_weight_time_left` - Weight coefficient for time remaining in ad
///   recommendations (0.0 to 1.0)
/// * `gpt_temperature` - Controls randomness in GPT responses (0.0 to 1.0)
/// * `gpt_max_tokens` - Maximum number of tokens in GPT response
#[derive(Clone)]
pub struct AppState {
    pub yandex_api_key: String,
    pub yandex_folder_id: String,
    pub system_prompt_for_generate_title: String,
    pub system_prompt_for_generate_body: String,

    pub ads_weight_profit: f64,
    pub ads_weight_relevance: f64,
    pub ads_weight_fulfillment: f64,
    pub ads_weight_time_left: f64,

    pub gpt_temperature: f32,
    pub gpt_max_tokens: u32,

    pub media_support_mime: Vec<String>,
    pub media_max_size: usize,
    pub media_max_image_on_campaign: usize,

    pub auto_moderating_sensitivity: f32,
}

/// Implements conversion from Config to AppState
///
/// Transforms the infrastructure configuration into application state,
/// copying all necessary fields and cloning String values where needed.
impl From<&infrastructure::configurate::Config> for AppState {
    fn from(config: &infrastructure::configurate::Config) -> Self {
        Self {
            yandex_api_key: config.yandex.api_key.clone(),
            yandex_folder_id: config.yandex.folder_id.clone(),
            ads_weight_profit: config.ads_recommendation.weight_profit,
            ads_weight_relevance: config.ads_recommendation.weight_relevance,
            ads_weight_fulfillment: config.ads_recommendation.weight_fulfillment,
            ads_weight_time_left: config.ads_recommendation.weight_time_left,
            gpt_temperature: config.yandex.gpt.temperature,
            gpt_max_tokens: config.yandex.gpt.max_tokens,
            system_prompt_for_generate_title: config.yandex.gpt.system_prompt_for_generate_title.clone(),
            system_prompt_for_generate_body: config.yandex.gpt.system_prompt_for_generate_body.clone(),
            media_support_mime: config.upload_content.support_mime.clone(),
            media_max_size: config.upload_content.max_size,
            media_max_image_on_campaign: config.upload_content.max_image_on_campaign,
            auto_moderating_sensitivity: config.auto_moderating.sensitivity,
        }
    }
}
