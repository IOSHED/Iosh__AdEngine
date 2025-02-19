use crate::{domain, infrastructure};

/// Represents possible service-level errors that can occur in the application.
/// This enum encapsulates all error types that can be returned by service layer
/// operations.
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ServiceError {
    /// Error that occurs during data validation when input fails validation
    /// rules. Contains a descriptive message of what validation failed.
    #[error("Validation error - {0}")]
    Validation(String),

    /// Error that occurs during database operations when interacting with the
    /// repository layer. Wraps the underlying repository error to maintain
    /// error context.
    #[error("Database error - {0}")]
    Repository(infrastructure::repository::RepoError),

    /// Error that occurs during cash/payment processing operations.
    /// Contains details about what went wrong during the cash transaction.
    #[error("Cash error - {0}")]
    Cash(String),

    /// Error that occurs when GPT service fails to provide a response.
    /// Contains information about why the GPT request failed.
    #[error("Gpt not response - {0}")]
    GptNotResponse(String),

    /// Error that occurs when content is flagged by content
    /// moderation/censorship rules. Contains details about which content
    /// was not acceptable.
    #[error("Not acceptable words - {0}")]
    Censorship(String),

    /// Error that occurs during payload processing or validation.
    /// Contains information about what was wrong with the payload.
    #[error("Payload error - {0}")]
    PayloadError(String),

    /// Represents an unknown or unexpected error that doesn't fit other
    /// categories. Used as a fallback when the error type cannot be
    /// determined.
    #[error("Unknown error")]
    Unknown,
}

/// Implements conversion from validator::ValidationErrors to ServiceError.
/// This implementation handles the mapping of validation framework errors to
/// our domain-specific error types.
///
/// # Arguments
/// * `value` - The ValidationErrors instance to convert
///
/// # Returns
/// * `ServiceError` - The corresponding service error with appropriate context

impl From<validator::ValidationErrors> for domain::services::ServiceError {
    fn from(value: validator::ValidationErrors) -> Self {
        if let Some((field, kind)) = value.errors().iter().next() {
            return match kind {
                validator::ValidationErrorsKind::Struct(errors) => ServiceError::handle_struct_error(field, errors),
                validator::ValidationErrorsKind::Field(errors) => ServiceError::handle_field_error(field, errors),
                validator::ValidationErrorsKind::List(errors) => ServiceError::handle_list_error(field, errors),
            };
        }
        tracing::error!(
            "Unknown error in `impl From<ValidationErrors> for domain::services::ServiceError` with args: {:#?}",
            value
        );
        domain::services::ServiceError::Unknown
    }
}

impl ServiceError {
    /// Handles validation errors that occur in struct fields
    ///
    /// # Arguments
    /// * `field` - The name of the field where the error occurred
    /// * `errors` - The validation errors associated with the struct
    fn handle_struct_error(field: &str, errors: &validator::ValidationErrors) -> ServiceError {
        if let Some((_, first_error)) = errors.errors().iter().next() {
            return ServiceError::Validation(format!("error in {}: {:?}", field, first_error));
        }
        ServiceError::Unknown
    }

    /// Handles validation errors that occur in individual fields
    ///
    /// # Arguments
    /// * `field` - The name of the field where the error occurred
    /// * `errors` - Array of validation errors for the field
    fn handle_field_error(field: &str, errors: &[validator::ValidationError]) -> ServiceError {
        if let Some(error) = errors.get(0) {
            return ServiceError::Validation(format!(
                "error in {}: {}",
                field,
                error
                    .message
                    .as_ref()
                    .unwrap_or(&std::borrow::Cow::from("Unknown error"))
            ));
        }
        ServiceError::Unknown
    }

    /// Handles validation errors that occur in list/array fields
    ///
    /// # Arguments
    /// * `field` - The name of the field where the error occurred
    /// * `errors` - Map of validation errors indexed by list position
    fn handle_list_error(
        field: &str,
        errors: &std::collections::BTreeMap<usize, Box<validator::ValidationErrors>>,
    ) -> ServiceError {
        if let Some((_index, first_error)) = errors.iter().next() {
            return ServiceError::Validation(format!("error in {}: {}", field, first_error));
        }
        ServiceError::Unknown
    }
}
