use crate::{
    domain,
    infrastructure::{self, repository::IRepo},
};

pub struct AdvertiserProfileUsecase<'p> {
    advertiser_service: domain::services::AdvertiserService,
    db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
}

impl<'p> AdvertiserProfileUsecase<'p> {
    pub fn new(db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool) -> Self {
        Self {
            advertiser_service: domain::services::AdvertiserService,
            db_pool,
        }
    }

    pub async fn get_by_id(
        self,
        advertiser_id: uuid::Uuid,
    ) -> domain::services::ServiceResult<domain::schemas::AdvertiserProfileSchema> {
        self.advertiser_service
            .get_by_id(
                advertiser_id,
                infrastructure::repository::sqlx_lib::PgAdvertiserRepository::new(self.db_pool),
            )
            .await
    }
}
