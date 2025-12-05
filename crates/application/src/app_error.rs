use domain::error::{domain_error::DomainError, persistence_error::PersistenceError};
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum AppError {
    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Resource already exist: {0}")]
    ResourceAlreadyExist(String),

    #[error("Generic error: {0}")]
    GenericError(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Forbidden")]
    Forbidden,

    #[error("Database error: {0}")]
    Database(String),

    #[error("Invalid focus session duration")]
    InvalidFocusSessionDuration,

    #[error("Invalid color: {0}")]
    InvalidColor(String),

    #[error("Invalid focus session parameter: {0}")]
    InvalidFocusSessionParam(String),

    #[error("Invalid date range: {0}")]
    InvalidDateRange(String),

    #[error("Invalid id: {0}")]
    InvalidId(String),

    #[error("Invalid stats parameter: {0}")]
    InvalidStatsParam(String),

    #[error("Persistance error: {0}")]
    Persistence(String),
}

impl From<DomainError> for AppError {
    fn from(error: DomainError) -> Self {
        match error {
            DomainError::InvalidFocusSessionDuration => AppError::InvalidFocusSessionDuration,
            DomainError::InvalidColor(msg) => AppError::InvalidColor(msg),
            DomainError::InvalidFocusSessionParam(msg) => AppError::InvalidFocusSessionParam(msg),
            DomainError::InvalidStatsParam(msg) => AppError::InvalidStatsParam(msg),
        }
    }
}

impl From<PersistenceError> for AppError {
    fn from(error: PersistenceError) -> Self {
        match error {
            PersistenceError::NotFound(msg) => AppError::NotFound(msg),
            PersistenceError::AlreadyExists => {
                AppError::GenericError("Resource already exists".to_string())
            }
            PersistenceError::Unexpected(msg) => AppError::Database(msg),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_error_display() {
        let error = AppError::NotFound("test".to_string());
        assert_eq!(error.to_string(), "Resource not found: test");

        let error = AppError::ResourceAlreadyExist("test".to_string());
        assert_eq!(error.to_string(), "Resource already exist: test");

        let error = AppError::GenericError("test".to_string());
        assert_eq!(error.to_string(), "Generic error: test");

        let error = AppError::BadRequest("test".to_string());
        assert_eq!(error.to_string(), "Bad request: test");

        let error = AppError::Forbidden;
        assert_eq!(error.to_string(), "Forbidden");

        let error = AppError::Database("test".to_string());
        assert_eq!(error.to_string(), "Database error: test");
    }
}
