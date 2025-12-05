use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum DomainError {
    #[error("Invalid focus session duration")]
    InvalidFocusSessionDuration,
}

pub type DomainResult<T> = Result<T, DomainError>;
