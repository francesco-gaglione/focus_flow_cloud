use thiserror::Error;

#[derive(Debug, Error)]
pub enum HashingError {
    #[error("Invalid hash: {0}")]
    InvalidHash(String),

    #[error("Invalid salt: {0}")]
    InvalidSalt(String),
}

pub type HashingResult<T> = Result<T, HashingError>;

#[cfg_attr(test, mockall::automock)]
pub trait PasswordHasher: Send + Sync {
    fn hash_password(&self, password: &str) -> HashingResult<String>;

    fn verify_password(&self, password: &str, hashed_password: &str) -> HashingResult<bool>;
}
