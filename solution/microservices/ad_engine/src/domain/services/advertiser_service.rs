use async_trait::async_trait;

use crate::{domain, infrastructure};

#[async_trait]
pub trait IRegisterBulkAdvertiser {
    async fn register(
        &self,
        advertiser_ids: Vec<uuid::Uuid>,
        names: Vec<String>,
    ) -> infrastructure::repository::RepoResult<Vec<infrastructure::repository::sqlx_lib::AdvertiserReturningSchema>>;
}

#[async_trait]
pub trait IGetAdvertiserById {
    async fn get_by_id(
        &self,
        advertiser_id: uuid::Uuid,
    ) -> infrastructure::repository::RepoResult<infrastructure::repository::sqlx_lib::AdvertiserReturningSchema>;
}

#[derive(std::fmt::Debug)]
pub struct AdvertiserService<'p> {
    db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool,
}

impl<'p> AdvertiserService<'p> {
    pub fn new(db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool) -> Self {
        Self { db_pool }
    }
}

impl<'p> AdvertiserService<'p> {
    #[tracing::instrument(name = "`UserService` register bulk Advertisers")]
    pub async fn register(
        self,
        register_data: Vec<domain::schemas::AdvertiserProfileSchema>,
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

        let repo_user = infrastructure::repository::sqlx_lib::PgAdvertiserRepository::new(self.db_pool)
            .register(advertiser_ids, names)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;

        Ok(repo_user.into_iter().map(|user| user.into()).collect())
    }

    #[tracing::instrument(name = "`UserService` get Advertiser by id")]
    pub async fn get_by_id(
        self,
        advertiser_id: uuid::Uuid,
    ) -> domain::services::ServiceResult<domain::schemas::AdvertiserProfileSchema> {
        let repo_user = infrastructure::repository::sqlx_lib::PgAdvertiserRepository::new(self.db_pool)
            .get_by_id(advertiser_id)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;

        Ok(repo_user.into())
    }
}
