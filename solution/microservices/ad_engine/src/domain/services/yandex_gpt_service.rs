use crate::{domain, infrastructure};

/// YandexGptService handles text generation using Yandex GPT API
/// for campaign titles and body content
#[derive(Debug)]
pub struct YandexGptService {
    gpt_client: infrastructure::gpt::yandex::YandexGptClient,
    system_prompt_for_generate_title: String,
    system_prompt_for_generate_body: String,
}

impl YandexGptService {
    /// Creates a new YandexGptService instance
    ///
    /// # Arguments
    /// * `folder_id` - Yandex folder ID for API access
    /// * `auth_token` - Authentication token for Yandex API
    /// * `temperature` - Temperature parameter for text generation (0.0-1.0)
    /// * `max_tokens` - Maximum number of tokens in generated response
    /// * `system_prompt_for_generate_title` - System prompt for title
    ///   generation
    /// * `system_prompt_for_generate_body` - System prompt for body text
    ///   generation
    pub fn new(
        folder_id: String,
        auth_token: String,
        temperature: f32,
        max_tokens: u32,
        system_prompt_for_generate_title: String,
        system_prompt_for_generate_body: String,
    ) -> Self {
        Self {
            gpt_client: infrastructure::gpt::yandex::YandexGptClient::new(
                folder_id,
                auth_token,
                temperature,
                max_tokens,
            ),
            system_prompt_for_generate_title,
            system_prompt_for_generate_body,
        }
    }

    /// Generates text content for a campaign based on specified generation type
    ///
    /// # Arguments
    /// * `campaign` - Mutable reference to campaign schema to update
    /// * `generate_schema` - Schema containing generation parameters
    ///
    /// # Returns
    /// * `ServiceResult<()>` - Result indicating success or error
    pub async fn generate_text_for_campaign(
        &self,
        campaign: &mut domain::schemas::CampaignSchema,
        generate_schema: domain::schemas::CampaignsGenerateTextRequest,
    ) -> domain::services::ServiceResult<()> {
        match generate_schema.generate_type.as_str() {
            "ALL" => {
                campaign.ad_text = self
                    .generate_body(&generate_schema.ad_text.unwrap_or(campaign.ad_text.clone()))
                    .await?;

                campaign.ad_title = self
                    .generate_title(&generate_schema.ad_title.unwrap_or(campaign.ad_title.clone()))
                    .await?;
            },

            "TITLE" => {
                campaign.ad_title = self
                    .generate_title(&generate_schema.ad_title.unwrap_or(campaign.ad_title.clone()))
                    .await?;
            },
            "TEXT" => {
                campaign.ad_text = self
                    .generate_body(&generate_schema.ad_text.unwrap_or(campaign.ad_text.clone()))
                    .await?;
            },
            _ =>
                return Err(domain::services::ServiceError::Validation(
                    "Invalid generate type".to_string(),
                )),
        }
        Ok(())
    }

    /// Generates campaign title using Yandex GPT
    ///
    /// # Arguments
    /// * `text` - Input text for title generation
    ///
    /// # Returns
    /// * `ServiceResult<String>` - Generated title or error
    pub async fn generate_title(&self, text: &str) -> domain::services::ServiceResult<String> {
        self.gpt_client
            .ask_gpt(text, &self.system_prompt_for_generate_title)
            .await
            .map_err(|e| domain::services::ServiceError::GptNotResponse(e.to_string()))
    }

    /// Generates campaign body text using Yandex GPT
    ///
    /// # Arguments  
    /// * `text` - Input text for body generation
    ///
    /// # Returns
    /// * `ServiceResult<String>` - Generated body text or error
    pub async fn generate_body(&self, text: &str) -> domain::services::ServiceResult<String> {
        self.gpt_client
            .ask_gpt(text, &self.system_prompt_for_generate_body)
            .await
            .map_err(|e| domain::services::ServiceError::GptNotResponse(e.to_string()))
    }
}
