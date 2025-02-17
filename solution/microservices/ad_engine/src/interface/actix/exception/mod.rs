use serde::Serialize;

use crate::{domain, infrastructure};

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ExceptionResponse {
    #[schema(example = "<Type Error> - <detail>")]
    reason: String,
}

impl ExceptionResponse {
    pub fn new(reason: String) -> Self {
        Self { reason }
    }
}

impl actix_web::error::ResponseError for domain::services::ServiceError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            domain::services::ServiceError::Repository(repo) => repo.into(),
            domain::services::ServiceError::Validation(_) => actix_web::http::StatusCode::BAD_REQUEST,
            domain::services::ServiceError::Unknown => actix_web::http::StatusCode::IM_A_TEAPOT,
            domain::services::ServiceError::Cash(_) => actix_web::http::StatusCode::IM_A_TEAPOT,
            domain::services::ServiceError::GptNotResponse(_) => actix_web::http::StatusCode::SERVICE_UNAVAILABLE,
            domain::services::ServiceError::Censorship(_) => actix_web::http::StatusCode::NOT_ACCEPTABLE,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code()).json(ExceptionResponse::new(self.to_string()))
    }
}

impl From<&infrastructure::repository::RepoError> for actix_web::http::StatusCode {
    fn from(value: &infrastructure::repository::RepoError) -> Self {
        match value {
            infrastructure::repository::RepoError::ObjDoesNotExists(_) => actix_web::http::StatusCode::NOT_FOUND,
            infrastructure::repository::RepoError::UniqueConstraint(_) => actix_web::http::StatusCode::CONFLICT,
            infrastructure::repository::RepoError::Unknown => actix_web::http::StatusCode::IM_A_TEAPOT,
        }
    }
}
