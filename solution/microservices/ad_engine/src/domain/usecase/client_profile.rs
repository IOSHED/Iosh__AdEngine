use crate::{domain, infrastructure};

pub struct ClientProfileUsecase<'p> {
    client_service: domain::services::ClientService<'p>,
}

impl<'p> ClientProfileUsecase<'p> {
    pub fn new(db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool) -> Self {
        Self {
            client_service: domain::services::ClientService::new(db_pool),
        }
    }

    pub async fn get_by_id(
        self,
        client_id: String,
    ) -> domain::services::ServiceResult<domain::schemas::ClientProfileSchema> {
        let client_id = uuid::Uuid::parse_str(&client_id)
            .map_err(|_| domain::services::ServiceError::Validation("uuid not valid".to_string()))?;
        self.client_service.get_by_id(client_id).await
    }
}
