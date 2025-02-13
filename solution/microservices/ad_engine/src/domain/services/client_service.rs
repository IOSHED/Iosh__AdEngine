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
pub struct ClientService;

impl<'p> ClientService {
    #[tracing::instrument(name = "`UserService` register bulk clients", skip(repo))]
    pub async fn register<R: infrastructure::repository::IRepo<'p> + IRegisterBulkClient>(
        &self,
        register_data: Vec<domain::schemas::ClientProfileSchema>,
        repo: R,
    ) -> domain::services::ServiceResult<Vec<domain::schemas::ClientProfileSchema>> {
        let mut clients_map: std::collections::HashMap<uuid::Uuid, domain::schemas::ClientProfileSchema> =
            std::collections::HashMap::new();

        for client in register_data {
            clients_map.insert(client.client_id, client);
        }

        let unique_clients: Vec<domain::schemas::ClientProfileSchema> = clients_map.into_values().collect();

        let (client_ids, logins, locations, genders, ages) = unique_clients.into_iter().fold(
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

        let repo_user = repo
            .register(client_ids, logins, locations, genders, ages)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;

        Ok(repo_user.into_iter().map(|user| user.into()).collect())
    }

    #[tracing::instrument(name = "`UserService` get client by id", skip(repo))]
    pub async fn get_by_id<R: infrastructure::repository::IRepo<'p> + IGetClientById>(
        &self,
        client_id: uuid::Uuid,
        repo: R,
    ) -> domain::services::ServiceResult<domain::schemas::ClientProfileSchema> {
        let repo_user = repo
            .get_by_id(client_id)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;

        Ok(repo_user.into())
    }
}
