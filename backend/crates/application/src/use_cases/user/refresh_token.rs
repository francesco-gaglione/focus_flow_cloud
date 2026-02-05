use domain::error::persistence_error::PersistenceError;
use domain::services::token_service::{TokenService, TokenServiceError};
use domain::traits::user_persistence::UserPersistence;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Error)]
pub enum RefreshTokenError {
    #[error("Token error: {0}")]
    TokenError(#[from] TokenServiceError),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type RefreshTokenResult<T> = Result<T, RefreshTokenError>;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RefreshTokenCommand {
    #[validate(length(min = 1, message = "Refresh token is required"))]
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenOutput {
    pub token: String,
    pub refresh_token: String,
}

pub struct RefreshTokenUseCase {
    user_persistence: Arc<dyn UserPersistence>,
    token_service: Arc<dyn TokenService>,
}

impl RefreshTokenUseCase {
    pub fn new(
        user_persistence: Arc<dyn UserPersistence>,
        token_service: Arc<dyn TokenService>,
    ) -> Self {
        Self {
            user_persistence,
            token_service,
        }
    }

    pub async fn execute(
        &self,
        cmd: RefreshTokenCommand,
    ) -> RefreshTokenResult<RefreshTokenOutput> {
        cmd.validate()
            .map_err(|e| RefreshTokenError::Validation(e.to_string()))?;

        // Verify refresh token
        let user_id_str = self
            .token_service
            .verify_refresh_token(&cmd.refresh_token)?;

        let user_id = uuid::Uuid::parse_str(&user_id_str)
            .map_err(|_| RefreshTokenError::InvalidCredentials)?;

        // Find user
        let user = self.user_persistence.find_user_by_id(user_id).await?;

        // Generate new tokens
        let token = self.token_service.generate_token(&user)?;

        let refresh_token = self.token_service.generate_refresh_token(&user)?;

        Ok(RefreshTokenOutput {
            token,
            refresh_token,
        })
    }
}

//TODO unit tests
