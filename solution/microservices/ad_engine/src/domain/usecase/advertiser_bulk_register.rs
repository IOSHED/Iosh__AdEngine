use validator::Validate;

use crate::{
    domain,
    infrastructure::{self, repository::IRepo},
};

pub struct AdvertiserBulkRegisterUsecase<'p> {
    advertiser_service: domain::services::AdvertiserService,
    moderate_text_service: domain::services::ModerateTextService,
    db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
    redis_pool: &'p infrastructure::database_connection::redis::RedisPool,
}

impl<'p> AdvertiserBulkRegisterUsecase<'p> {
    pub fn new(
        db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
        redis_pool: &'p infrastructure::database_connection::redis::RedisPool,
    ) -> Self {
        Self {
            advertiser_service: domain::services::AdvertiserService,
            moderate_text_service: domain::services::ModerateTextService,
            db_pool,
            redis_pool,
        }
    }

    pub async fn registers(
        self,
        register_data: Vec<domain::schemas::AdvertiserProfileSchema>,
    ) -> domain::services::ServiceResult<Vec<domain::schemas::AdvertiserProfileSchema>> {
        for register in &register_data {
            register
                .validate()
                .map_err(|e| domain::services::ServiceError::Validation(e.to_string()))?;

            self.moderate_text_service
                .check_abusive_content(
                    &[register.name.clone()],
                    infrastructure::repository::redis::RedisObsceneWordRepository::new(self.redis_pool, self.db_pool),
                )
                .await?;
        }

        let advertisers = self
            .advertiser_service
            .register(
                register_data,
                infrastructure::repository::sqlx_lib::PgAdvertiserRepository::new(self.db_pool),
            )
            .await?;

        domain::services::PrometheusService::add_total_advertisers(advertisers.len() as i64);

        Ok(advertisers)
    }
}
