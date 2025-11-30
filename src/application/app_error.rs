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
