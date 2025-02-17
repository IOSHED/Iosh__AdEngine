use anyhow::Context;

mod schemas;

#[derive(Debug)]
pub struct YandexGptClient {
    client: reqwest::Client,
    folder_id: String,
    auth_token: String,
    temperature: f32,
    max_tokens: u32,
}

impl YandexGptClient {
    pub fn new(folder_id: String, auth_token: String, temperature: f32, max_tokens: u32) -> Self {
        Self {
            client: reqwest::Client::new(),
            folder_id,
            auth_token,

            temperature,
            max_tokens,
        }
    }

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
