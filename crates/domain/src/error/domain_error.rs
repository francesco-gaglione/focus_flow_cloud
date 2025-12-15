use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum DomainError {
    #[error("Invalid focus session duration")]
    InvalidFocusSessionDuration,

    #[error("Invalid focus session parameters: {0}")]
    InvalidFocusSessionParam(String),

    #[error("Invalid color: {0}")]
    InvalidColor(String),

    #[error("Invalid stats parameter: {0}")]
    InvalidStatsParam(String),

    #[error("Invalid password policy: {0}")]
    InvalidPasswordPolicy(String),

    #[error("Password hashing error: {0}")]
    PasswordHashingError(String),
}

pub type DomainResult<T> = Result<T, DomainError>;
