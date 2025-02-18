use serde::Serialize;

use crate::{domain, infrastructure};

pub struct RedisService<'p> {
    repo: infrastructure::cash::redis::RedisExecutor<'p>,
}

impl<'p> RedisService<'p> {
    pub fn new(pool: &'p infrastructure::database_connection::redis::RedisPool) -> Self {
        RedisService {
            repo: infrastructure::cash::redis::RedisExecutor::new(pool),
        }
    }

    pub async fn get_all_active_campaigns(
        &self,
    ) -> domain::services::ServiceResult<Vec<domain::schemas::ActiveCampaignSchema>> {
        let mut conn = self.repo.get_conn().await?;
        let mut cursor: isize = 0;
        let mut active_campaigns = Vec::new();
        let pattern = "active_campaign:*";

        loop {
            let result: redis::RedisResult<(isize, Vec<String>)> = redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg(pattern)
                .query(&mut conn);

            match result {
                Ok((new_cursor, keys)) => {
                    for key in keys {
                        let campaign: domain::schemas::ActiveCampaignSchema = self.repo.get(&key).await?;
                        active_campaigns.push(campaign);
                    }

                    cursor = new_cursor;

                    if cursor == 0 {
                        break;
                    }
                },
                Err(_) => {
                    return Err(domain::services::ServiceError::Cash("Redis SCAN error".to_string()));
                },
            }
        }

        Ok(active_campaigns)
    }

    async fn get_random_chunk_from_uuid(&self, id: &uuid::Uuid) -> String {
        id.to_string()
            .replace("-", "")
            .chars()
            .skip(0)
            .take(24)
            .collect::<String>()
    }

    pub async fn del_active_campaigns(&self, id: &uuid::Uuid) -> domain::services::ServiceResult<()> {
        let random_id = self.get_random_chunk_from_uuid(id).await;
        self.repo.delete(&format!("active_campaign:{random_id}")).await
    }

    pub async fn get_active_campaign(
        &self,
        id: &uuid::Uuid,
    ) -> domain::services::ServiceResult<domain::schemas::ActiveCampaignSchema> {
        let random_id = self.get_random_chunk_from_uuid(id).await;
        self.repo.get(&format!("active_campaign:{random_id}")).await
    }

    pub async fn set_active_campaign(
        &self,
        data: domain::schemas::ActiveCampaignSchema,
    ) -> domain::services::ServiceResult<()> {
        let random_id = self.get_random_chunk_from_uuid(&data.campaign_id).await;
        self.repo.set(&format!("active_campaign:{random_id}"), data).await
    }

    pub async fn get_obscene_words(&self) -> domain::services::ServiceResult<Vec<String>> {
        let word: String = self.repo.get("obscene_words").await?;
        Ok(word.split(',').map(|s| s.to_string()).collect())
    }

    pub async fn set_obscene_words(&self, data: Vec<String>) -> domain::services::ServiceResult<()> {
        self.repo.set("obscene_words", data.join(",").to_string()).await
    }

    pub async fn get_advance_time(&self) -> domain::services::ServiceResult<u32> {
        match self.repo.get("advance_time").await {
            Ok(data) => Ok(data),
            Err(_) => {
                self.set_advance_time(0 as u32).await?;
                self.repo.get("advance_time").await
            },
        }
    }

    pub async fn set_advance_time(&self, data: u32) -> domain::services::ServiceResult<()> {
        self.repo.set("advance_time", data).await
    }

    pub async fn get_is_activate_auto_moderate(&self) -> domain::services::ServiceResult<bool> {
        match self.repo.get("is_activate_auto_moderate").await {
            Ok(data) => Ok(data),
            Err(_) => {
                self.set_is_activate_auto_moderate(false).await?;
                self.repo.get("is_activate_auto_moderate").await
            },
        }
    }

    pub async fn set_is_activate_auto_moderate(&self, data: bool) -> domain::services::ServiceResult<()> {
        self.repo.set("is_activate_auto_moderate", data).await
    }
}

impl redis::ToRedisArgs for domain::schemas::ActiveCampaignSchema {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        let mut buf = Vec::new();
        self.serialize(&mut rmp_serde::Serializer::new(&mut buf))
            .expect("Failed to serialize ActiveCampaignSchema to MessagePack");

        out.write_arg(&buf);
    }
}

impl redis::FromRedisValue for domain::schemas::ActiveCampaignSchema {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        match v {
            redis::Value::BulkString(data) => {
                let campaign: domain::schemas::ActiveCampaignSchema = rmp_serde::from_slice(&data).map_err(|_| {
                    redis::RedisError::from((redis::ErrorKind::TypeError, "Failed to deserialize bincode"))
                })?;
                Ok(campaign)
            },
            _ => Err(redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "Expected a binary string value",
            ))),
        }
    }
}
