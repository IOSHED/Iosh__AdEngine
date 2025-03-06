#[derive(Debug, serde::Deserialize)]
pub struct IamTokenResponse {
    #[serde(rename = "iamToken")]
    pub iam_token: String,
}

#[derive(Debug, serde::Serialize)]
pub struct GenerationOptions {
    pub temperature: f32,
    #[serde(rename = "maxTokens")]
    pub max_tokens: u32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Message {
    pub role: String,
    pub text: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct GptResponse {
    pub result: GptResult,
}

#[derive(Debug, serde::Deserialize)]
pub struct GptResult {
    pub alternatives: Vec<Alternative>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Alternative {
    pub message: Message,
}
