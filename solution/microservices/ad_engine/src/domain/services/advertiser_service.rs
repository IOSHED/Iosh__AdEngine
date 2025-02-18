use async_trait::async_trait;

use crate::{domain, infrastructure};

/// Trait for bulk registration of advertisers in the system.
/// Implementations should handle the persistence of multiple advertisers in a
/// single operation.
#[async_trait]
pub trait IRegisterBulkAdvertiser {
    /// Registers multiple advertisers simultaneously
    ///
    /// # Arguments
    /// * `advertiser_ids` - Vector of UUIDs for the advertisers to register
    /// * `names` - Vector of names corresponding to the advertiser IDs
    ///
    /// # Returns
    /// * `RepoResult<Vec<AdvertiserReturningSchema>>` - Result containing
    ///   vector of created advertisers or repository error
    async fn register(
        &self,
        advertiser_ids: Vec<uuid::Uuid>,
        names: Vec<String>,
    ) -> infrastructure::repository::RepoResult<Vec<infrastructure::repository::sqlx_lib::AdvertiserReturningSchema>>;
}

/// Trait for retrieving individual advertiser records by their unique
/// identifier.
#[async_trait]
pub trait IGetAdvertiserById {
    /// Retrieves a single advertiser by their UUID
    ///
    /// # Arguments
    /// * `advertiser_id` - UUID of the advertiser to retrieve
    ///
    /// # Returns
    /// * `RepoResult<AdvertiserReturningSchema>` - Result containing the found
    ///   advertiser or repository error
    async fn get_by_id(
        &self,
        advertiser_id: uuid::Uuid,
    ) -> infrastructure::repository::RepoResult<infrastructure::repository::sqlx_lib::AdvertiserReturningSchema>;
}

/// Service struct handling advertiser-related business logic
#[derive(std::fmt::Debug)]
pub struct AdvertiserService;

impl<'p> AdvertiserService {
    /// Registers multiple advertisers while handling deduplication based on
    /// advertiser ID
    ///
    /// # Arguments
    /// * `register_data` - Vector of advertiser profile schemas to register
    /// * `repo` - Repository implementation handling the persistence
    ///
    /// # Returns
    /// * `ServiceResult<Vec<AdvertiserProfileSchema>>` - Result containing
    ///   registered advertisers or service error
    ///
    /// # Example
    /// ```rust,no_run
    /// let service = AdvertiserService;
    /// let advertisers = vec![AdvertiserProfileSchema {
    ///     advertiser_id: uuid::Uuid::new_v4(),
    ///     name: "Test Advertiser".to_string(),
    /// }];
    /// let result = service.register(advertisers, repository).await?;
    /// ```
    #[tracing::instrument(name = "`UserService` register bulk Advertisers", skip(repo))]
    pub async fn register<R: IRegisterBulkAdvertiser>(
        &self,
        register_data: Vec<domain::schemas::AdvertiserProfileSchema>,
        repo: R,
    ) -> domain::services::ServiceResult<Vec<domain::schemas::AdvertiserProfileSchema>> {
        let mut advertisers_map: std::collections::HashMap<uuid::Uuid, domain::schemas::AdvertiserProfileSchema> =
            std::collections::HashMap::new();

        for advertiser in register_data {
            advertisers_map.insert(advertiser.advertiser_id, advertiser);
        }

        let unique_advertisers: Vec<domain::schemas::AdvertiserProfileSchema> = advertisers_map.into_values().collect();

        let (advertiser_ids, names) =
            unique_advertisers
                .into_iter()
                .fold((Vec::new(), Vec::new()), |(mut uuids, mut names), advertiser| {
                    uuids.push(advertiser.advertiser_id);
                    names.push(advertiser.name);
                    (uuids, names)
                });

        let repo_user = repo
            .register(advertiser_ids, names)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;

        Ok(repo_user.into_iter().map(|user| user.into()).collect())
    }

    /// Retrieves a single advertiser by their UUID
    ///
    /// # Arguments
    /// * `advertiser_id` - UUID of the advertiser to retrieve
    /// * `repo` - Repository implementation handling the data access
    ///
    /// # Returns
    /// * `ServiceResult<AdvertiserProfileSchema>` - Result containing found
    ///   advertiser or service error
    ///
    /// # Example
    /// ```rust,no_run
    /// let service = AdvertiserService;
    /// let advertiser_id = uuid::Uuid::new_v4();
    /// let advertiser = service.get_by_id(advertiser_id, repository).await?;
    /// ```
    #[tracing::instrument(name = "`UserService` get Advertiser by id", skip(repo))]
    pub async fn get_by_id<R: IGetAdvertiserById>(
        &self,
        advertiser_id: uuid::Uuid,
        repo: R,
    ) -> domain::services::ServiceResult<domain::schemas::AdvertiserProfileSchema> {
        let repo_user = repo
            .get_by_id(advertiser_id)
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
        pub RegisterBulkAdvertiser {}
        #[async_trait]
        impl IRegisterBulkAdvertiser for RegisterBulkAdvertiser {
            async fn register(
                &self,
                advertiser_ids: Vec<Uuid>,
                names: Vec<String>,
            ) -> infrastructure::repository::RepoResult
            <Vec<infrastructure::repository::sqlx_lib::AdvertiserReturningSchema>>;
        }
    }

    mock! {
        pub GetAdvertiserById {}
        #[async_trait]
        impl IGetAdvertiserById for GetAdvertiserById {
            async fn get_by_id(
                &self,
                advertiser_id: Uuid,
            ) -> infrastructure::repository::RepoResult
            <infrastructure::repository::sqlx_lib::AdvertiserReturningSchema>;
        }
    }

    #[tokio::test]
    async fn test_register_bulk_advertisers() {
        let mut mock_repo = MockRegisterBulkAdvertiser::new();

        let advertiser_id_1 = Uuid::new_v4();
        let advertiser_id_2 = Uuid::new_v4();

        let advertiser_1 = domain::schemas::AdvertiserProfileSchema {
            advertiser_id: advertiser_id_1,
            name: "Advertiser 1".to_string(),
        };

        let advertiser_2 = domain::schemas::AdvertiserProfileSchema {
            advertiser_id: advertiser_id_2,
            name: "Advertiser 2".to_string(),
        };

        let advertiser_3 = domain::schemas::AdvertiserProfileSchema {
            advertiser_id: advertiser_id_1,
            name: "Advertiser 1 Duplicate".to_string(),
        };

        let input_data = vec![advertiser_1.clone(), advertiser_2.clone(), advertiser_3.clone()];

        mock_repo
            .expect_register()
            .withf(move |ids, names| {
                ids.len() == 2
                    && names.len() == 2
                    && ids.contains(&advertiser_id_1)
                    && ids.contains(&advertiser_id_2)
                    && names.contains(&"Advertiser 1 Duplicate".to_string())
                    && names.contains(&"Advertiser 2".to_string())
            })
            .returning(move |_, _| {
                Ok(vec![
                    infrastructure::repository::sqlx_lib::AdvertiserReturningSchema {
                        advertiser_id: advertiser_id_1,
                        name: "Advertiser 1 Duplicate".to_string(),
                    },
                    infrastructure::repository::sqlx_lib::AdvertiserReturningSchema {
                        advertiser_id: advertiser_id_2,
                        name: "Advertiser 2".to_string(),
                    },
                ])
            });

        let service = AdvertiserService;
        let result = service.register(input_data, mock_repo).await;

        assert!(result.is_ok());
        let returned_advertisers = result.unwrap();
        assert_eq!(returned_advertisers.len(), 2);

        let expected_advertiser_3 = domain::schemas::AdvertiserProfileSchema {
            advertiser_id: advertiser_id_1,
            name: "Advertiser 1 Duplicate".to_string(),
        };

        assert!(returned_advertisers.contains(&expected_advertiser_3));
        assert!(returned_advertisers.contains(&advertiser_2));
    }
    #[tokio::test]
    async fn test_get_advertiser_by_id() {
        let mut mock_repo = MockGetAdvertiserById::new();

        let advertiser_id = Uuid::new_v4();
        let expected_advertiser = domain::schemas::AdvertiserProfileSchema {
            advertiser_id,
            name: "Test Advertiser".to_string(),
        };

        mock_repo
            .expect_get_by_id()
            .with(eq(advertiser_id))
            .returning(move |_| {
                Ok(infrastructure::repository::sqlx_lib::AdvertiserReturningSchema {
                    advertiser_id,
                    name: "Test Advertiser".to_string(),
                })
            });

        let service = AdvertiserService;
        let result = service.get_by_id(advertiser_id, mock_repo).await;

        assert!(result.is_ok());
        let returned_advertiser = result.unwrap();
        assert_eq!(returned_advertiser, expected_advertiser);
    }

    #[tokio::test]
    async fn test_register_bulk_advertisers_with_repo_error() {
        let mut mock_repo = MockRegisterBulkAdvertiser::new();

        let advertiser_id = Uuid::new_v4();
        let advertiser = domain::schemas::AdvertiserProfileSchema {
            advertiser_id,
            name: "Advertiser 1".to_string(),
        };

        let input_data = vec![advertiser];

        mock_repo
            .expect_register()
            .returning(|_, _| Err(infrastructure::repository::RepoError::UniqueConstraint("err".into())));

        let service = AdvertiserService;
        let result = service.register(input_data, mock_repo).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            domain::services::ServiceError::Repository(_) => (),
            _ => panic!("Expected Repository error"),
        }
    }
}
