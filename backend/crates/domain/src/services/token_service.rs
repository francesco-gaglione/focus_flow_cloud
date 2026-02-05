use crate::entities::user::User;
use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenServiceError {
    #[error("Invalid token")]
    InvalidToken,
    #[error("Invalid refresh token")]
    InvalidRefreshToken,
    #[error("Token generation error: {0}")]
    TokenGenerationError(String),
    #[error("Token verification error: {0}")]
    TokenVerificationError(String),
}

pub type TokenServiceResult<T> = Result<T, TokenServiceError>;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait TokenService: Send + Sync {
    fn generate_token(&self, user: &User) -> TokenServiceResult<String>;
    fn generate_refresh_token(&self, user: &User) -> TokenServiceResult<String>;
    fn verify_token(&self, token: &str) -> TokenServiceResult<String>;
    fn verify_refresh_token(&self, token: &str) -> TokenServiceResult<String>;
}
