use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum PersistenceError {
    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Entity already exists")]
    AlreadyExists,

    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

pub type PersistenceResult<T> = Result<T, PersistenceError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_error_display() {
        let error = PersistenceError::NotFound("test".to_string());
        assert_eq!(error.to_string(), "Resource not found: test");
    }
}
