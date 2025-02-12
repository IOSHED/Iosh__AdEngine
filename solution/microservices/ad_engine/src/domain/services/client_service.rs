use async_trait::async_trait;

use crate::{domain, infrastructure};

#[async_trait]
pub trait IRegisterBulkClient {
    async fn register(
        &self,
        client_ids: Vec<uuid::Uuid>,
        logins: Vec<String>,
        locations: Vec<String>,
        genders: Vec<String>,
        ages: Vec<i32>,
    ) -> infrastructure::repository::RepoResult<Vec<infrastructure::repository::sqlx_lib::ClientReturningSchema>>;
}

#[async_trait]
pub trait IGetClientById {
    async fn get_by_id(
        &self,
        client_id: uuid::Uuid,
    ) -> infrastructure::repository::RepoResult<infrastructure::repository::sqlx_lib::ClientReturningSchema>;
}

#[derive(std::fmt::Debug)]
pub struct ClientService<'p> {
    db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
}

impl<'p> ClientService<'p> {
    pub fn new(db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool) -> Self {
        Self { db_pool }
    }
}

impl<'p> ClientService<'p> {
    #[tracing::instrument(name = "`UserService` register bulk clients")]
    pub async fn register(
        self,
        register_data: Vec<domain::schemas::UserProfileSchema>,
    ) -> domain::services::ServiceResult<Vec<domain::schemas::UserProfileSchema>> {
        let (client_ids, logins, locations, genders, ages): (
            Vec<uuid::Uuid>,
            Vec<String>,
            Vec<String>,
            Vec<String>,
            Vec<i32>,
        ) = register_data
            .into_iter()
            .map(|client| {
                (
                    // Use unwrap because in highest we validation this field
                    uuid::Uuid::parse_str(&client.client_id).unwrap(),
                    client.login,
                    client.location,
                    client.gender,
                    client.age as i32,
                )
            })
            .collect();

        let repo_user = infrastructure::repository::sqlx_lib::PgClientRepository::new(self.db_pool)
            .register(client_ids, logins, locations, genders, ages)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;

        Ok(repo_user.into_iter().map(|user| user.into()).collect())
    }

    #[tracing::instrument(name = "`UserService` get client by id")]
    pub async fn get_by_id(
        self,
        client_id: uuid::Uuid,
    ) -> domain::services::ServiceResult<domain::schemas::UserProfileSchema> {
        let repo_user = infrastructure::repository::sqlx_lib::PgClientRepository::new(self.db_pool)
            .get_by_id(client_id)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;

        Ok(repo_user.into())
    }
}

/// Implements conversion from repository user schema to domain user profile
/// schema.
impl From<infrastructure::repository::sqlx_lib::ClientReturningSchema> for domain::schemas::UserProfileSchema {
    fn from(user: infrastructure::repository::sqlx_lib::ClientReturningSchema) -> Self {
        Self {
            client_id: user.client_id,
            login: user.login,
            location: user.location,
            gender: user.gender,
            age: user.age as u8,
        }
    }
}
