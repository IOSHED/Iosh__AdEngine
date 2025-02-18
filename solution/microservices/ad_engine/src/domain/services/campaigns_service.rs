use async_trait::async_trait;
use bigdecimal::ToPrimitive;

use crate::{domain, infrastructure};

#[async_trait]
pub trait ICreateCampaign {
    async fn create(
        &self,
        campaign: domain::schemas::CampaignsCreateRequest,
        advertiser_id: uuid::Uuid,
    ) -> infrastructure::repository::RepoResult<infrastructure::repository::sqlx_lib::CampaignReturningSchema>;
}

#[async_trait]
pub trait IUpdateCampaign {
    async fn update(
        &self,
        campaign: domain::schemas::CampaignsUpdateRequest,
        advertiser_id: uuid::Uuid,
        campaign_id: uuid::Uuid,
    ) -> infrastructure::repository::RepoResult<infrastructure::repository::sqlx_lib::CampaignReturningSchema>;
}

#[async_trait]
pub trait IGetCampaignById {
    async fn get_by_id(
        &self,
        advertiser_id: uuid::Uuid,
        campaign_id: uuid::Uuid,
    ) -> infrastructure::repository::RepoResult<infrastructure::repository::sqlx_lib::CampaignReturningSchema>;
}

#[async_trait]
pub trait IGetCampaignList {
    async fn get_list(
        &self,
        advertiser_id: uuid::Uuid,
        size: u32,
        page: u32,
    ) -> infrastructure::repository::RepoResult<(u64, Vec<infrastructure::repository::sqlx_lib::CampaignReturningSchema>)>;
}

#[async_trait]
pub trait IGetActiveCampaignList {
    async fn get_active_campaigns(
        &self,
        current_date: u32,
    ) -> infrastructure::repository::RepoResult<Vec<infrastructure::repository::sqlx_lib::CampaignReturningSchema>>;
}

#[async_trait]
pub trait IDeleteCampaign {
    async fn delete(
        &self,
        advertiser_id: uuid::Uuid,
        campaign_id: uuid::Uuid,
    ) -> infrastructure::repository::RepoResult<()>;
}

#[async_trait]
pub trait IGetIdsCampaign {
    async fn get_campaign_ids(
        &self,
        advertiser_id: uuid::Uuid,
    ) -> infrastructure::repository::RepoResult<Vec<uuid::Uuid>>;
}

#[async_trait]
pub trait ISearchCampaign {
    async fn are_exist(&self, campaign_id: uuid::Uuid) -> infrastructure::repository::RepoResult<bool>;
}

#[derive(Debug)]
pub struct CampaignService;

impl<'p> CampaignService {
    #[tracing::instrument(name = "`CampaignService` create campaign", skip(repo))]
    pub async fn create<R: ICreateCampaign>(
        &self,
        campaign: domain::schemas::CampaignsCreateRequest,
        advertiser_id: uuid::Uuid,
        repo: R,
    ) -> domain::services::ServiceResult<domain::schemas::CampaignSchema> {
        let repo_campaign = repo
            .create(campaign, advertiser_id)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;

        Ok(repo_campaign.into())
    }

    #[tracing::instrument(name = "`CampaignService` update campaign", skip(repo))]
    pub async fn update<R: IUpdateCampaign + IGetCampaignById>(
        &self,
        campaign: domain::schemas::CampaignsUpdateRequest,
        advertiser_id: uuid::Uuid,
        campaign_id: uuid::Uuid,
        time_advance: u32,
        repo: R,
    ) -> domain::services::ServiceResult<domain::schemas::CampaignSchema> {
        let old_campaign: domain::schemas::CampaignSchema = repo
            .get_by_id(advertiser_id, campaign_id)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?
            .into();

        if (time_advance >= campaign.start_date)
            & ((old_campaign.impressions_limit != campaign.impressions_limit)
                | (old_campaign.clicks_limit != campaign.clicks_limit)
                | (old_campaign.end_date != campaign.end_date)
                | (old_campaign.start_date != campaign.start_date))
        {
            return Err(domain::services::ServiceError::Validation(
                "Fields impressions_limit, clicks_limit, end_date, start_date don't update before start compaign"
                    .into(),
            ));
        }

        let repo_campaign = repo
            .update(campaign, advertiser_id, campaign_id)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;

        Ok(repo_campaign.into())
    }

    #[tracing::instrument(name = "`CampaignService` delete campaign", skip(repo))]
    pub async fn delete<R: IDeleteCampaign + IGetCampaignById>(
        &self,
        advertiser_id: uuid::Uuid,
        campaign_id: uuid::Uuid,
        repo: R,
    ) -> domain::services::ServiceResult<()> {
        let campaign = repo
            .get_by_id(advertiser_id, campaign_id)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;

        repo.delete(campaign.advertiser_id, campaign.id)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;

        Ok(())
    }

    #[tracing::instrument(name = "`CampaignService` get campaign by id", skip(repo))]
    pub async fn get_by_id<R: IGetCampaignById>(
        &self,
        advertiser_id: uuid::Uuid,
        campaign_id: uuid::Uuid,
        repo: R,
    ) -> domain::services::ServiceResult<domain::schemas::CampaignSchema> {
        let repo_campaign = repo
            .get_by_id(advertiser_id, campaign_id)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;

        Ok(repo_campaign.into())
    }

    #[tracing::instrument(name = "`CampaignService` get list of campaigns", skip(repo))]
    pub async fn get_list<R: IGetCampaignList>(
        &self,
        advertiser_id: uuid::Uuid,
        size: u32,
        page: u32,
        repo: R,
    ) -> domain::services::ServiceResult<(u64, Vec<domain::schemas::CampaignSchema>)> {
        let (total_count, campaigns) = repo
            .get_list(advertiser_id, size, page)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;

        let campaigns = campaigns.into_iter().map(|c| c.into()).collect();

        Ok((total_count, campaigns))
    }

    #[tracing::instrument(name = "`CampaignService` get list of campaigns", skip(repo))]
    pub async fn get_active_campaigns<R: IGetActiveCampaignList>(
        &self,
        current_date: u32,
        repo: R,
    ) -> domain::services::ServiceResult<Vec<domain::schemas::CampaignSchema>> {
        let campaign = repo
            .get_active_campaigns(current_date)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;
        Ok(campaign.into_iter().map(|c| c.into()).collect())
    }

    #[tracing::instrument(name = "`CampaignService` are exist campaign", skip(repo))]
    pub async fn campaign_is_exist<R: ISearchCampaign>(
        &self,
        campaign_id: uuid::Uuid,
        repo: R,
    ) -> domain::services::ServiceResult<bool> {
        repo.are_exist(campaign_id)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))
    }

    #[tracing::instrument(name = "`CampaignService` are exist campaign", skip(repo))]
    pub async fn get_campaign_ids<R: IGetIdsCampaign>(
        &self,
        advertiser_id: uuid::Uuid,
        repo: R,
    ) -> domain::services::ServiceResult<Vec<uuid::Uuid>> {
        repo.get_campaign_ids(advertiser_id)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))
    }
}

impl From<infrastructure::repository::sqlx_lib::CampaignReturningSchema> for domain::schemas::CampaignSchema {
    fn from(campaign: infrastructure::repository::sqlx_lib::CampaignReturningSchema) -> Self {
        Self {
            campaign_id: campaign.id,
            advertiser_id: campaign.advertiser_id,
            impressions_limit: campaign.impressions_limit as u32,
            clicks_limit: campaign.clicks_limit as u32,
            cost_per_impressions: campaign.cost_per_impressions.to_f64().unwrap_or(0.0),
            cost_per_clicks: campaign.cost_per_clicks.to_f64().unwrap_or(0.0),
            ad_title: campaign.ad_title,
            ad_text: campaign.ad_text,
            start_date: campaign.start_date as u32,
            end_date: campaign.end_date as u32,
            targeting: serde_json::from_value(campaign.targeting.unwrap_or(serde_json::json!({})))
                .unwrap_or(domain::schemas::TargetingCampaignSchema::default()),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use async_trait::async_trait;
//     use mockall::{mock, predicate::*};
//     use uuid::Uuid;

//     use super::*;

//     mock! {
//         pub CreateCampaign {}
//         #[async_trait]
//         impl ICreateCampaign for CreateCampaign {
//             async fn create(
//                 &self,
//                 campaign: domain::schemas::CampaignsCreateRequest,
//                 advertiser_id: Uuid,
//                 created_at: u32,
//             ) ->
// infrastructure::repository::RepoResult<domain::schemas::CampaignSchema>;
//         }
//     }

//     mock! {
//         pub UpdateCampaign {}
//         #[async_trait]
//         impl IUpdateCampaign for UpdateCampaign {
//             async fn update(
//                 &self,
//                 campaign: domain::schemas::CampaignsUpdateRequest,
//                 advertiser_id: Uuid,
//                 campaign_id: Uuid,
//                 update_at: u32,
//             ) ->
// infrastructure::repository::RepoResult<domain::schemas::CampaignSchema>;
//         }
//         #[async_trait]
//         impl IGetCampaignById for UpdateCampaign {
//             async fn get_by_id(
//                 &self,
//                 advertiser_id: Uuid,
//                 campaign_id: Uuid,
//             ) ->
// infrastructure::repository::RepoResult<domain::schemas::CampaignSchema>;
//         }
//     }

//     mock! {
//         pub GetCampaignById {}
//         #[async_trait]
//         impl IGetCampaignById for GetCampaignById {
//             async fn get_by_id(
//                 &self,
//                 advertiser_id: Uuid,
//                 campaign_id: Uuid,
//             ) ->
// infrastructure::repository::RepoResult<domain::schemas::CampaignSchema>;
//         }
//     }

//     mock! {
//         pub GetCampaignList {}
//         #[async_trait]
//         impl IGetCampaignList for GetCampaignList {
//             async fn get_list(
//                 &self,
//                 advertiser_id: Uuid,
//                 size: u32,
//                 page: u32,
//             ) -> infrastructure::repository::RepoResult<(u64,
// Vec<domain::schemas::CampaignSchema>)>;         }
//     }

//     mock! {
//         pub DeleteCampaign {}
//         #[async_trait]
//         impl IDeleteCampaign for DeleteCampaign {
//             async fn delete(
//                 &self,
//                 advertiser_id: Uuid,
//                 campaign_id: Uuid,
//             ) -> infrastructure::repository::RepoResult<()>;
//         }
//     }

//     #[tokio::test]
//     async fn test_create_campaign() {
//         let mut mock_repo = MockCreateCampaign::new();

//         let advertiser_id = Uuid::new_v4();
//         let campaign_request = domain::schemas::CampaignsCreateRequest {
//             impressions_limit: 100,
//             clicks_limit: 200,
//             cost_per_impressions: 10.0,
//             cost_per_clicks: 20.0,
//             ad_title: "Test Campaign".to_string(),
//             ad_text: "This is a test campaign".to_string(),
//             start_date: 12345,
//             end_date: 123456,
//             targeting: domain::schemas::TargetingCampaignSchema {
//                 gender: Some("MALE".to_string()),
//                 age_from: Some(18),
//                 age_to: Some(35),
//                 location: Some("Moscow".to_string()),
//             },
//         };

//         let expected_campaign = domain::schemas::CampaignSchema {
//             campaign_id: Uuid::new_v4(),
//             advertiser_id,
//             impressions_limit: 100,
//             clicks_limit: 200,
//             cost_per_impressions: 10.0,
//             cost_per_clicks: 20.0,
//             ad_title: "Test Campaign".to_string(),
//             ad_text: "This is a test campaign".to_string(),
//             start_date: 12345,
//             end_date: 123456,
//             targeting: domain::schemas::TargetingCampaignSchema {
//                 gender: Some("MALE".to_string()),
//                 age_from: Some(18),
//                 age_to: Some(35),
//                 location: Some("Moscow".to_string()),
//             },
//         };
//         let expected_campaign_mock = expected_campaign.clone();

//         mock_repo
//             .expect_create()
//             .with(eq(campaign_request.clone()), eq(advertiser_id), eq(12345))
//             .returning(move |_, _, _| Ok(expected_campaign_mock.clone()));

//         let service = CampaignService;
//         let result = service.create(campaign_request, advertiser_id, 12345,
// mock_repo).await;

//         assert!(result.is_ok());
//         let returned_campaign = result.unwrap();
//         assert_eq!(returned_campaign, expected_campaign);
//     }

//     #[tokio::test]
//     async fn test_update_campaign() {
//         let mut mock_repo = MockUpdateCampaign::new();

//         let advertiser_id = Uuid::new_v4();
//         let campaign_id = Uuid::new_v4();
//         let time_advance = 12345;

//         let campaign_request = domain::schemas::CampaignsUpdateRequest {
//             impressions_limit: 100,
//             clicks_limit: 200,
//             cost_per_impressions: 15.0,
//             cost_per_clicks: 25.0,
//             ad_title: "Updated Campaign".to_string(),
//             ad_text: "This is an updated campaign".to_string(),
//             start_date: 12346,
//             end_date: 123456,
//             targeting: domain::schemas::TargetingCampaignSchema {
//                 gender: Some("FEMALE".to_string()),
//                 age_from: Some(20),
//                 age_to: Some(40),
//                 location: Some("St. Petersburg".to_string()),
//             },
//         };

//         let old_campaign = domain::schemas::CampaignSchema {
//             campaign_id,
//             advertiser_id,
//             impressions_limit: 100,
//             clicks_limit: 200,
//             cost_per_impressions: 10.0,
//             cost_per_clicks: 20.0,
//             ad_title: "Test Campaign".to_string(),
//             ad_text: "This is a test campaign".to_string(),
//             start_date: 12345,
//             end_date: 123456,
//             targeting: domain::schemas::TargetingCampaignSchema {
//                 gender: Some("MALE".to_string()),
//                 age_from: Some(18),
//                 age_to: Some(35),
//                 location: Some("Moscow".to_string()),
//             },
//         };

//         let expected_campaign = domain::schemas::CampaignSchema {
//             campaign_id,
//             advertiser_id,
//             impressions_limit: 100,
//             clicks_limit: 200,
//             cost_per_impressions: 15.0,
//             cost_per_clicks: 25.0,
//             ad_title: "Updated Campaign".to_string(),
//             ad_text: "This is an updated campaign".to_string(),
//             start_date: 12346,
//             end_date: 123456,
//             targeting: domain::schemas::TargetingCampaignSchema {
//                 gender: Some("FEMALE".to_string()),
//                 age_from: Some(20),
//                 age_to: Some(40),
//                 location: Some("St. Petersburg".to_string()),
//             },
//         };
//         let expected_campaign_moc = expected_campaign.clone();

//         mock_repo
//             .expect_get_by_id()
//             .with(eq(advertiser_id), eq(campaign_id))
//             .returning(move |_, _| Ok(old_campaign.clone()));

//         mock_repo
//             .expect_update()
//             .with(
//                 eq(campaign_request.clone()),
//                 eq(advertiser_id),
//                 eq(campaign_id),
//                 eq(time_advance),
//             )
//             .returning(move |_, _, _, _| Ok(expected_campaign_moc.clone()));

//         let service = CampaignService;
//         let result = service
//             .update(campaign_request, advertiser_id, campaign_id,
// time_advance, mock_repo)             .await;

//         assert!(result.is_ok());
//         let returned_campaign = result.unwrap();
//         assert_eq!(returned_campaign, expected_campaign);
//     }

//     #[tokio::test]
//     async fn test_delete_campaign() {
//         let mut mock_repo = MockDeleteCampaign::new();

//         let advertiser_id = Uuid::new_v4();
//         let campaign_id = Uuid::new_v4();

//         mock_repo
//             .expect_delete()
//             .with(eq(advertiser_id), eq(campaign_id))
//             .returning(|_, _| Ok(()));

//         let service = CampaignService;
//         let result = service.delete(advertiser_id, campaign_id,
// mock_repo).await;

//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_get_campaign_by_id() {
//         let mut mock_repo = MockGetCampaignById::new();

//         let advertiser_id = Uuid::new_v4();
//         let campaign_id = Uuid::new_v4();
//         let expected_campaign = domain::schemas::CampaignSchema {
//             campaign_id,
//             advertiser_id,
//             impressions_limit: 100,
//             clicks_limit: 200,
//             cost_per_impressions: 10.0,
//             cost_per_clicks: 20.0,
//             ad_title: "Test Campaign".to_string(),
//             ad_text: "This is a test campaign".to_string(),
//             start_date: 12345,
//             end_date: 123456,
//             targeting: domain::schemas::TargetingCampaignSchema {
//                 gender: Some("MALE".to_string()),
//                 age_from: Some(18),
//                 age_to: Some(35),
//                 location: Some("Moscow".to_string()),
//             },
//         };

//         let expected_campaign_mock = expected_campaign.clone();

//         mock_repo
//             .expect_get_by_id()
//             .with(eq(advertiser_id), eq(campaign_id))
//             .returning(move |_, _| Ok(expected_campaign_mock.clone()));

//         let service = CampaignService;
//         let result = service.get_by_id(advertiser_id, campaign_id,
// mock_repo).await;

//         assert!(result.is_ok());
//         let returned_campaign = result.unwrap();
//         assert_eq!(returned_campaign, expected_campaign);
//     }

//     #[tokio::test]
//     async fn test_get_campaign_list() {
//         let mut mock_repo = MockGetCampaignList::new();

//         let advertiser_id = Uuid::new_v4();
//         let campaigns = vec![
//             domain::schemas::CampaignSchema {
//                 campaign_id: Uuid::new_v4(),
//                 advertiser_id,
//                 impressions_limit: 100,
//                 clicks_limit: 200,
//                 cost_per_impressions: 10.0,
//                 cost_per_clicks: 20.0,
//                 ad_title: "Campaign 1".to_string(),
//                 ad_text: "This is campaign 1".to_string(),
//                 start_date: 12345,
//                 end_date: 123456,
//                 targeting: domain::schemas::TargetingCampaignSchema {
//                     gender: Some("MALE".to_string()),
//                     age_from: Some(18),
//                     age_to: Some(35),
//                     location: Some("Moscow".to_string()),
//                 },
//             },
//             domain::schemas::CampaignSchema {
//                 campaign_id: Uuid::new_v4(),
//                 advertiser_id,
//                 impressions_limit: 150,
//                 clicks_limit: 250,
//                 cost_per_impressions: 15.0,
//                 cost_per_clicks: 25.0,
//                 ad_title: "Campaign 2".to_string(),
//                 ad_text: "This is campaign 2".to_string(),
//                 start_date: 12345,
//                 end_date: 123456,
//                 targeting: domain::schemas::TargetingCampaignSchema {
//                     gender: Some("FEMALE".to_string()),
//                     age_from: Some(20),
//                     age_to: Some(40),
//                     location: Some("St. Petersburg".to_string()),
//                 },
//             },
//         ];

//         mock_repo
//             .expect_get_list()
//             .with(eq(advertiser_id), eq(10), eq(1))
//             .returning(move |_, _, _| Ok((2, campaigns.clone())));

//         let service = CampaignService;
//         let result = service.get_list(advertiser_id, 10, 1, mock_repo).await;

//         assert!(result.is_ok());
//         let (total_count, returned_campaigns) = result.unwrap();
//         assert_eq!(total_count, 2);
//         assert_eq!(returned_campaigns.len(), 2);
//     }
// }
