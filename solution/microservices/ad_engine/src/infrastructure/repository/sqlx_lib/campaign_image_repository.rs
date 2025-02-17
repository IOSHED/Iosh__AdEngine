use async_trait::async_trait;

use crate::{domain, infrastructure};

#[derive(Debug)]
pub struct PgCampaignImageRepository<'p> {
    db_pool: &'p sqlx::Pool<sqlx::Postgres>,
}

impl<'p> infrastructure::repository::IRepo<'p> for PgCampaignImageRepository<'p> {
    fn new(db_pool: &'p sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl<'p> domain::services::repository::IUploadCampaignImage for PgCampaignImageRepository<'p> {
    async fn upload(
        &self,
        campaign_id: uuid::Uuid,
        media_max_image_on_campaign: usize,
        files: Vec<(String, Vec<u8>, String)>,
    ) -> infrastructure::repository::RepoResult<()> {
        let mut transaction = self.db_pool.begin().await?;

        let current_count: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM campaigns_images WHERE campaign_id = $1",
            campaign_id
        )
        .fetch_one(&mut *transaction)
        .await?
        .unwrap_or(0);

        if current_count as usize + files.len() > media_max_image_on_campaign {
            return Err(infrastructure::repository::RepoError::UniqueConstraint(
                "maximum 5 images per campaign".into(),
            ));
        }

        for (file_name, data, mime_type) in files {
            let file_size = data.len() as i64;
            
            sqlx::query!(
                r#"
                INSERT INTO campaigns_images 
                    (data, mime_type, file_name, file_size, campaign_id)
                VALUES 
                    ($1, $2, $3, $4, $5)
                "#,
                data,
                mime_type,
                file_name,
                file_size,
                campaign_id
            )
            .execute(&mut *transaction)
            .await?;
        }

        transaction.commit().await?;
        Ok(())
    }
}
