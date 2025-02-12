use crate::{domain, infrastructure};

pub struct AdvertiserProfileUsecase<'p> {
    advertiser_service: domain::services::AdvertiserService<'p>,
}

impl<'p> AdvertiserProfileUsecase<'p> {
    pub fn new(db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool) -> Self {
        Self {
            advertiser_service: domain::services::AdvertiserService::new(db_pool),
        }
    }

    pub async fn get_by_id(
        self,
        advertiser_id: String,
    ) -> domain::services::ServiceResult<domain::schemas::AdvertiserProfileSchema> {
        let advertiser_id = uuid::Uuid::parse_str(&advertiser_id)
            .map_err(|_| domain::services::ServiceError::Validation("uuid not valid".to_string()))?;
        self.advertiser_service.get_by_id(advertiser_id).await
    }
}
