use serde::Serialize;

use crate::{domain, infrastructure};

/// Represents an error response structure for API exceptions
///
/// This structure is used to provide standardized error responses across the
/// API, containing a human-readable reason for the error.
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ExceptionResponse {
    #[schema(example = "<Type Error> - <detail>")]
    reason: String,
}

impl ExceptionResponse {
    /// Creates a new ExceptionResponse instance
    ///
    /// # Arguments
    /// * `reason` - A string describing the error reason
    pub fn new(reason: String) -> Self {
        Self { reason }
    }
}

/// Implementation of ResponseError trait for ServiceError
///
/// Maps domain service errors to appropriate HTTP status codes and formats
/// error responses according to API standards.
impl actix_web::error::ResponseError for domain::services::ServiceError {
    /// Maps service errors to HTTP status codes
    ///
    /// # Returns
    /// * `StatusCode` - The appropriate HTTP status code for the error
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            domain::services::ServiceError::Repository(repo) => repo.into(),
            domain::services::ServiceError::Validation(_) => actix_web::http::StatusCode::BAD_REQUEST,
            domain::services::ServiceError::Unknown => actix_web::http::StatusCode::IM_A_TEAPOT,
            domain::services::ServiceError::Cash(_) => actix_web::http::StatusCode::IM_A_TEAPOT,
            domain::services::ServiceError::GptNotResponse(_) => actix_web::http::StatusCode::SERVICE_UNAVAILABLE,
            domain::services::ServiceError::Censorship(_) => actix_web::http::StatusCode::NOT_ACCEPTABLE,
            domain::services::ServiceError::PayloadError(_) => actix_web::http::StatusCode::BAD_REQUEST,
        }
    }

    /// Builds an HTTP response for the error
    ///
    /// # Returns
    /// * `HttpResponse` - JSON response containing error details
    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code()).json(ExceptionResponse::new(self.to_string()))
    }
}

/// Conversion implementation from repository errors to HTTP status codes
///
/// Provides mapping between domain repository errors and their corresponding
/// HTTP status codes for consistent error handling.
impl From<&infrastructure::repository::RepoError> for actix_web::http::StatusCode {
    fn from(value: &infrastructure::repository::RepoError) -> Self {
        match value {
            infrastructure::repository::RepoError::ObjDoesNotExists(_) => actix_web::http::StatusCode::NOT_FOUND,
            infrastructure::repository::RepoError::UniqueConstraint(_) => actix_web::http::StatusCode::CONFLICT,
            infrastructure::repository::RepoError::Unknown => actix_web::http::StatusCode::IM_A_TEAPOT,
        }
    }
}
