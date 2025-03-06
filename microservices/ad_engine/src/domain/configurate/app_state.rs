use crate::infrastructure;

/// Configuration state for the application's core functionality
///
/// Encapsulates all configuration parameters needed for the application's
/// operation, including Yandex GPT integration, content generation settings, ad
/// recommendation algorithms, and media handling constraints.
///
/// # Configuration Categories
///
/// ## Yandex GPT Integration
/// * `yandex_api_key` - API authentication key for Yandex services
/// * `yandex_folder_id` - Resource folder identifier in Yandex cloud
/// * `gpt_temperature` - Controls output randomness (0.0 = deterministic, 1.0 =
///   creative)
/// * `gpt_max_tokens` - Response length limit in tokens
///
/// ## Content Generation
/// * `system_prompt_for_generate_title` - Template prompt for AI title
///   generation
/// * `system_prompt_for_generate_body` - Template prompt for AI body text
///   generation
///
/// ## Ad Recommendation Weights
/// All weights are normalized values between 0.0 and 1.0:
/// * `ads_weight_profit` - Profit optimization factor
/// * `ads_weight_relevance` - Content relevance factor
/// * `ads_weight_fulfillment` - Delivery success factor
/// * `ads_weight_time_left` - Time urgency factor
///
/// ## Media Handling
/// * `media_support_mime` - List of supported MIME types
/// * `media_max_size` - Maximum allowed file size in bytes
/// * `media_max_image_on_campaign` - Image limit per campaign
///
/// ## Content Moderation
/// * `auto_moderating_sensitivity` - Sensitivity threshold for auto-moderation
///   (0.0 to 1.0)
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

/// Provides conversion from infrastructure Config to AppState
///
/// # Implementation Details
///
/// This implementation handles the transformation of the raw configuration data
/// into a properly structured application state. It performs:
/// - Deep cloning of String values to ensure ownership
/// - Direct copying of primitive values
/// - Preservation of all configuration hierarchies
///
/// # Arguments
///
/// * `config` - Reference to a Config instance containing raw configuration
///   data
///
/// # Returns
///
/// Returns a new AppState instance populated with the configuration values
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
