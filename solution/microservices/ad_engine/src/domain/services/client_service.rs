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
        register_data: Vec<domain::schemas::ClientProfileSchema>,
    ) -> domain::services::ServiceResult<Vec<domain::schemas::ClientProfileSchema>> {
        let (client_ids, logins, locations, genders, ages) = register_data.into_iter().fold(
            (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()),
            |(mut uuids, mut names, mut emails, mut phones, mut ages), client| {
                uuids.push(client.client_id);
                names.push(client.login);
                emails.push(client.location);
                phones.push(client.gender);
                ages.push(client.age as i32);
                (uuids, names, emails, phones, ages)
            },
        );

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
    ) -> domain::services::ServiceResult<domain::schemas::ClientProfileSchema> {
        let repo_user = infrastructure::repository::sqlx_lib::PgClientRepository::new(self.db_pool)
            .get_by_id(client_id)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;

        Ok(repo_user.into())
    }
}
