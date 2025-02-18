use anyhow::Context;

mod schemas;

/// A client for interacting with Yandex GPT API.
///
/// This client handles authentication and communication with the Yandex GPT
/// service, allowing you to send prompts and receive AI-generated responses.
#[derive(Debug)]
pub struct YandexGptClient {
    client: reqwest::Client,
    folder_id: String,
    auth_token: String,
    temperature: f32,
    max_tokens: u32,
}

impl YandexGptClient {
    /// Creates a new instance of YandexGptClient.
    ///
    /// # Arguments
    ///
    /// * `folder_id` - The Yandex Cloud folder ID where the GPT model is
    ///   located
    /// * `auth_token` - OAuth token for authentication with Yandex Cloud
    /// * `temperature` - Controls randomness in the model's output (0.0 to 1.0)
    /// * `max_tokens` - Maximum number of tokens in the generated response
    ///
    /// # Returns
    ///
    /// Returns a new `YandexGptClient` instance configured with the provided
    /// parameters.
    pub fn new(folder_id: String, auth_token: String, temperature: f32, max_tokens: u32) -> Self {
        Self {
            client: reqwest::Client::new(),
            folder_id,
            auth_token,

            temperature,
            max_tokens,
        }
    }

    /// Retrieves an IAM token for API authentication.
    ///
    /// Makes a request to Yandex IAM service to exchange the OAuth token for an
    /// IAM token.
    ///
    /// # Returns
    ///
    /// Returns a Result containing the IAM token string or an error if the
    /// request fails.
    async fn get_iam_token(&self) -> anyhow::Result<String> {
        let response = self
            .client
            .post("https://iam.api.cloud.yandex.net/iam/v1/tokens")
            .json(&serde_json::json!({"yandexPassportOauthToken": self.auth_token.clone()}))
            .send()
            .await
            .context("Failed to send IAM token request")?;

        let token_response: schemas::IamTokenResponse =
            response.json().await.context("Failed to parse IAM token response")?;

        Ok(token_response.iam_token)
    }

    /// Sends a prompt to Yandex GPT and returns the generated response.
    ///
    /// # Arguments
    ///
    /// * `user_prompt` - The main prompt text to send to the model
    /// * `system_prompt` - System-level instructions that guide the model's
    ///   behavior
    ///
    /// # Returns
    ///
    /// Returns a Result containing the generated response text or an error if
    /// the request fails.

    pub async fn ask_gpt(&self, user_prompt: &str, system_prompt: &str) -> anyhow::Result<String> {
        let iam_token = self.get_iam_token().await?;

        let request_body = serde_json::json!({
            "modelUri": format!("gpt://{}/yandexgpt", self.folder_id),
            "generationOptions": schemas::GenerationOptions {
                temperature: self.temperature,
                max_tokens: self.max_tokens,
            },
            "messages": vec![
                schemas::Message {
                    role: "system".to_string(),
                    text: system_prompt.to_string(),
                },
                schemas::Message {
                    role: "user".to_string(),
                    text: user_prompt.to_string(),
                },
            ],
        });

        tracing::info!("Sending GPT request: {}", format!("Bearer {}", iam_token));

        let response = self
            .client
            .post("https://llm.api.cloud.yandex.net/foundationModels/v1/completion")
            .header("Authorization", format!("Bearer {}", iam_token))
            .json(&request_body)
            .send()
            .await
            .context("Failed to send GPT request")?;

        let gpt_response: schemas::GptResponse = response.json().await.context("Failed to parse GPT response")?;

        Ok(gpt_response.result.alternatives[0].message.text.clone())
    }
}
