use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordPolicyError {
    #[error("Length rule: {0}")]
    InvalidLength(String),

    #[error("Password must contain at least one uppercase letter")]
    MissingUppercase,

    #[error("Password must contain at least one lowercase letter")]
    MissingLowercase,

    #[error("Password must contain at least one number")]
    MissingNumber,
}

pub type PasswordPolicyResult<T> = Result<T, PasswordPolicyError>;

pub trait PasswordPolicy: Send + Sync {
    fn validate(&self, password: &str) -> PasswordPolicyResult<()>;
}
