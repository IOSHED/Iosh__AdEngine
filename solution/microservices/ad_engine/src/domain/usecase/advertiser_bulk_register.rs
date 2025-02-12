use validator::Validate;

use crate::{domain, infrastructure};

pub struct AdvertiserBulkRegisterUsecase<'p> {
    advertiser_service: domain::services::AdvertiserService<'p>,
}

impl<'p> AdvertiserBulkRegisterUsecase<'p> {
    pub fn new(db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool) -> Self {
        Self {
            advertiser_service: domain::services::AdvertiserService::new(db_pool),
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

            uuid::Uuid::parse_str(&register.advertiser_id)
                .map_err(|_| domain::services::ServiceError::Validation("uuid not valid".to_string()))?;
        }

        self.advertiser_service.register(register_data).await
    }
}
