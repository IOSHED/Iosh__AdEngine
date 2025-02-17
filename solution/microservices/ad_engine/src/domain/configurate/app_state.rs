use crate::infrastructure;

/// Represents the application's state that can be shared across different parts
/// of the system.
///
/// This struct is designed to hold any shared resources or configuration that
/// needs to be accessible throughout the application's lifetime.
#[derive(Clone, Debug)]
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
}

impl From<&infrastructure::configurate::Config> for AppState {
    /// Creates a new `AppState` instance from a configuration reference.
    ///
    /// # Arguments
    ///
    /// * `_config` - A reference to the infrastructure configuration
    ///
    /// # Returns
    ///
    /// Returns a new instance of `AppState`
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
        }
    }
}
