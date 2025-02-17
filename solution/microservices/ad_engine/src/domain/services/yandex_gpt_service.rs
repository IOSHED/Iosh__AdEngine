use crate::{domain, infrastructure};

#[derive(Debug)]

pub struct YandexGptService {
    gpt_client: infrastructure::gpt::yandex::YandexGptClient,
    system_prompt_for_generate_title: String,
    system_prompt_for_generate_body: String,
}

impl YandexGptService {
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

    pub async fn generate_text_for_campaign(
        &self,
        campaign: &mut domain::schemas::CampaignSchema,
        generate_schema: domain::schemas::CampaignsGenerateTextRequest,
    ) -> domain::services::ServiceResult<()> {
        match generate_schema.generate_type.as_str() {
            "ALL" => {
                campaign.ad_text = self
                    .generate_body(
                        &generate_schema.ad_text.unwrap_or(campaign.ad_text.clone()),
                        campaign.targeting.clone(),
                    )
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
                    .generate_body(
                        &generate_schema.ad_text.unwrap_or(campaign.ad_text.clone()),
                        campaign.targeting.clone(),
                    )
                    .await?;
            },
            _ =>
                return Err(domain::services::ServiceError::Validation(
                    "Invalid generate type".to_string(),
                )),
        }
        Ok(())
    }

    pub async fn generate_title(&self, text: &str) -> domain::services::ServiceResult<String> {
        self.gpt_client
            .ask_gpt(text, &self.system_prompt_for_generate_title)
            .await
            .map_err(|e| domain::services::ServiceError::GptNotResponse(e.to_string()))
    }

    pub async fn generate_body(
        &self,
        text: &str,
        target: domain::schemas::TargetingCampaignSchema,
    ) -> domain::services::ServiceResult<String> {
        let target_str = format!(
            "Возраст от {} до {}, в локации {}, для гендера {}",
            target.age_from.unwrap_or(0),
            target.age_to.unwrap_or(160),
            target.location.unwrap_or("любой".into()),
            target.gender.unwrap_or("любого".into()),
        );
        let system_prompt = self.system_prompt_for_generate_body.replace("{target}", &target_str);
        self.gpt_client
            .ask_gpt(text, &system_prompt)
            .await
            .map_err(|e| domain::services::ServiceError::GptNotResponse(e.to_string()))
    }
}
