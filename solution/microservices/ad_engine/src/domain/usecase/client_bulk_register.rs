use validator::Validate;

use crate::{domain, infrastructure};

pub struct ClientBulkRegisterUsecase<'p> {
    client_service: domain::services::ClientService<'p>,
}

impl<'p> ClientBulkRegisterUsecase<'p> {
    pub fn new(db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool) -> Self {
        Self {
            client_service: domain::services::ClientService::new(db_pool),
        }
    }

    pub async fn registers(
        self,
        register_data: Vec<domain::schemas::UserProfileSchema>,
    ) -> domain::services::ServiceResult<Vec<domain::schemas::UserProfileSchema>> {
        for register in &register_data {
            register
                .validate()
                .map_err(|e| domain::services::ServiceError::Validation(e.to_string()))?;

            uuid::Uuid::parse_str(&register.client_id)
                .map_err(|_| domain::services::ServiceError::Validation("uuid not valid".to_string()))?;
        }

        self.client_service.register(register_data).await
    }
}
