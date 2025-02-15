use async_trait::async_trait;

use crate::{domain, infrastructure};

#[async_trait]
pub trait ISetMlScore {
    async fn set_ml_score(
        &self,
        client_id: uuid::Uuid,
        advertiser_id: uuid::Uuid,
        score: f64,
    ) -> infrastructure::repository::RepoResult<()>;
}

#[derive(std::fmt::Debug)]
pub struct MlScoreService;

impl<'p> MlScoreService {
    #[tracing::instrument(name = "`MlScoreService` set ML score", skip(repo))]
    pub async fn set_ml_score<R: ISetMlScore>(
        &self,
        client_id: uuid::Uuid,
        advertiser_id: uuid::Uuid,
        score: f64,
        repo: R,
    ) -> domain::services::ServiceResult<()> {
        repo.set_ml_score(client_id, advertiser_id, score)
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use mockall::{mock, predicate::*};
    use uuid::Uuid;

    use super::*;

    mock! {
        pub SetMlScore {}
        #[async_trait]
        impl ISetMlScore for SetMlScore {
            async fn set_ml_score(
                &self,
                client_id: Uuid,
                advertiser_id: Uuid,
                score: f64,
            ) -> infrastructure::repository::RepoResult<()>;
        }
    }

    #[tokio::test]
    async fn test_set_ml_score() {
        let mut mock_repo = MockSetMlScore::new();

        let client_id = Uuid::new_v4();
        let advertiser_id = Uuid::new_v4();
        let score = 0.85;

        mock_repo
            .expect_set_ml_score()
            .with(eq(client_id), eq(advertiser_id), eq(score))
            .returning(|_, _, _| Ok(()));

        let service = MlScoreService;
        let result = service.set_ml_score(client_id, advertiser_id, score, mock_repo).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_ml_score_with_repo_error() {
        let mut mock_repo = MockSetMlScore::new();

        let client_id = Uuid::new_v4();
        let advertiser_id = Uuid::new_v4();
        let score = 0.85;

        mock_repo
            .expect_set_ml_score()
            .returning(|_, _, _| Err(infrastructure::repository::RepoError::Unknown));

        let service = MlScoreService;
        let result = service.set_ml_score(client_id, advertiser_id, score, mock_repo).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            domain::services::ServiceError::Repository(_) => (),
            _ => panic!("Expected Repository error"),
        }
    }
}
