use async_trait::async_trait;

use crate::{domain, infrastructure};

/// Trait defining the interface for bulk client registration operations.
/// Implementations should handle the registration of multiple clients
/// simultaneously.
#[async_trait]
pub trait IRegisterBulkClient {
    /// Registers multiple clients in bulk.
    ///
    /// # Arguments
    /// * `client_ids` - Vector of UUIDs for each client
    /// * `logins` - Vector of login names corresponding to each client
    /// * `locations` - Vector of location strings for each client
    /// * `genders` - Vector of gender strings for each client
    /// * `ages` - Vector of ages as integers for each client
    ///
    /// # Returns
    /// A Result containing a vector of registered client data on success,
    /// or a repository error on failure
    async fn register(
        &self,
        client_ids: Vec<uuid::Uuid>,
        logins: Vec<String>,
        locations: Vec<String>,
        genders: Vec<String>,
        ages: Vec<i32>,
    ) -> infrastructure::repository::RepoResult<Vec<infrastructure::repository::sqlx_lib::ClientReturningSchema>>;
}

/// Trait defining the interface for retrieving individual client data by ID.
#[async_trait]
pub trait IGetClientById {
    /// Retrieves a single client's data by their UUID.
    ///
    /// # Arguments
    /// * `client_id` - UUID of the client to retrieve
    ///
    /// # Returns
    /// A Result containing the client data on success,
    /// or a repository error if the client is not found or other errors occur
    async fn get_by_id(
        &self,
        client_id: uuid::Uuid,
    ) -> infrastructure::repository::RepoResult<infrastructure::repository::sqlx_lib::ClientReturningSchema>;
}

/// Service struct handling client-related business logic operations.
/// Provides methods for client registration and retrieval.
#[derive(std::fmt::Debug)]
pub struct ClientService;

impl<'p> ClientService {
    /// Registers multiple clients while handling potential duplicates.
    ///
    /// # Arguments
    /// * `register_data` - Vector of client profile data to register
    /// * `repo` - Repository implementation handling the actual storage
    ///
    /// # Returns
    /// A ServiceResult containing a vector of registered client profiles on
    /// success, or a service error on failure
    ///
    /// # Note
    /// This method automatically deduplicates clients based on their UUID
    /// before registration
    #[tracing::instrument(name = "`UserService` register bulk clients", skip(repo))]
    pub async fn register<R: IRegisterBulkClient>(
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

    /// Retrieves a single client by their UUID.
    ///
    /// # Arguments
    /// * `client_id` - UUID of the client to retrieve
    /// * `repo` - Repository implementation handling the data retrieval
    ///
    /// # Returns
    /// A ServiceResult containing the client profile on success,
    /// or a service error if the client is not found or other errors occur
    #[tracing::instrument(name = "`UserService` get client by id", skip(repo))]
    pub async fn get_by_id<R: IGetClientById>(
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

#[cfg(test)]
mod tests {

    use async_trait::async_trait;
    use mockall::{mock, predicate::*};
    use uuid::Uuid;

    use super::*;

    mock! {
        pub RegisterBulkClient {}
        #[async_trait]
        impl IRegisterBulkClient for RegisterBulkClient {
            async fn register(
                &self,
                client_ids: Vec<Uuid>,
                logins: Vec<String>,
                locations: Vec<String>,
                genders: Vec<String>,
                ages: Vec<i32>,
            ) -> infrastructure::repository::RepoResult
            <Vec<infrastructure::repository::sqlx_lib::ClientReturningSchema>>;
        }
    }

    mock! {
        pub GetClientById {}
        #[async_trait]
        impl IGetClientById for GetClientById {
            async fn get_by_id(
                &self,
                client_id: Uuid,
            ) -> infrastructure::repository::RepoResult<infrastructure::repository::sqlx_lib::ClientReturningSchema>;
        }
    }

    #[tokio::test]
    async fn test_register_bulk_clients() {
        let mut mock_repo = MockRegisterBulkClient::new();

        let client_id_1 = Uuid::new_v4();
        let client_id_2 = Uuid::new_v4();

        let client_1 = domain::schemas::ClientProfileSchema {
            client_id: client_id_1,
            login: "client1".to_string(),
            location: "Moscow".to_string(),
            gender: "MALE".to_string(),
            age: 25,
        };

        let client_2 = domain::schemas::ClientProfileSchema {
            client_id: client_id_2,
            login: "client2".to_string(),
            location: "St. Petersburg".to_string(),
            gender: "FEMALE".to_string(),
            age: 30,
        };

        let client_3 = domain::schemas::ClientProfileSchema {
            client_id: client_id_1,
            login: "client1_duplicate".to_string(),
            location: "Moscow".to_string(),
            gender: "MALE".to_string(),
            age: 25,
        };

        let input_data = vec![client_1.clone(), client_2.clone(), client_3.clone()];

        mock_repo
            .expect_register()
            .withf(move |ids, logins, locations, genders, ages| {
                ids.len() == 2
                    && logins.len() == 2
                    && locations.len() == 2
                    && genders.len() == 2
                    && ages.len() == 2
                    && ids.contains(&client_id_1)
                    && ids.contains(&client_id_2)
                    && logins.contains(&"client1_duplicate".to_string())
                    && logins.contains(&"client2".to_string())
                    && locations.contains(&"Moscow".to_string())
                    && locations.contains(&"St. Petersburg".to_string())
                    && genders.contains(&"MALE".to_string())
                    && genders.contains(&"FEMALE".to_string())
                    && ages.contains(&25)
                    && ages.contains(&30)
            })
            .returning(move |_, _, _, _, _| {
                Ok(vec![
                    infrastructure::repository::sqlx_lib::ClientReturningSchema {
                        client_id: client_id_1,
                        login: "client1_duplicate".to_string(),
                        location: "Moscow".to_string(),
                        gender: "MALE".to_string(),
                        age: 25,
                    },
                    infrastructure::repository::sqlx_lib::ClientReturningSchema {
                        client_id: client_id_2,
                        login: "client2".to_string(),
                        location: "St. Petersburg".to_string(),
                        gender: "FEMALE".to_string(),
                        age: 30,
                    },
                ])
            });

        let service = ClientService;
        let result = service.register(input_data, mock_repo).await;

        assert!(result.is_ok());
        let returned_clients = result.unwrap();
        assert_eq!(returned_clients.len(), 2);

        let expected_client_3 = domain::schemas::ClientProfileSchema {
            client_id: client_id_1,
            login: "client1_duplicate".to_string(),
            location: "Moscow".to_string(),
            gender: "MALE".to_string(),
            age: 25,
        };

        assert!(returned_clients.contains(&expected_client_3));
        assert!(returned_clients.contains(&client_2));
    }

    #[tokio::test]
    async fn test_get_client_by_id() {
        let mut mock_repo = MockGetClientById::new();

        let client_id = Uuid::new_v4();
        let expected_client = domain::schemas::ClientProfileSchema {
            client_id,
            login: "test_client".to_string(),
            location: "Moscow".to_string(),
            gender: "MALE".to_string(),
            age: 25,
        };

        mock_repo.expect_get_by_id().with(eq(client_id)).returning(move |_| {
            Ok(infrastructure::repository::sqlx_lib::ClientReturningSchema {
                client_id,
                login: "test_client".to_string(),
                location: "Moscow".to_string(),
                gender: "MALE".to_string(),
                age: 25,
            })
        });

        let service = ClientService;
        let result = service.get_by_id(client_id, mock_repo).await;

        assert!(result.is_ok());
        let returned_client = result.unwrap();
        assert_eq!(returned_client, expected_client);
    }

    #[tokio::test]
    async fn test_register_bulk_clients_with_repo_error() {
        let mut mock_repo = MockRegisterBulkClient::new();

        let client_id = Uuid::new_v4();
        let client = domain::schemas::ClientProfileSchema {
            client_id,
            login: "client1".to_string(),
            location: "Moscow".to_string(),
            gender: "MALE".to_string(),
            age: 25,
        };

        let input_data = vec![client];

        mock_repo
            .expect_register()
            .returning(|_, _, _, _, _| Err(infrastructure::repository::RepoError::UniqueConstraint("err".into())));

        let service = ClientService;
        let result = service.register(input_data, mock_repo).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            domain::services::ServiceError::Repository(_) => (),
            _ => panic!("Expected Repository error"),
        }
    }

    #[tokio::test]
    async fn test_get_client_by_id_with_repo_error() {
        let mut mock_repo = MockGetClientById::new();

        let client_id = Uuid::new_v4();

        mock_repo
            .expect_get_by_id()
            .returning(|_| Err(infrastructure::repository::RepoError::ObjDoesNotExists("err".into())));

        let service = ClientService;
        let result = service.get_by_id(client_id, mock_repo).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            domain::services::ServiceError::Repository(_) => (),
            _ => panic!("Expected Repository error"),
        }
    }
}
