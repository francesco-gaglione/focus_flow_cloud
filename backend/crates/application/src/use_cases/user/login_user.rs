use domain::services::token_service::{TokenService, TokenServiceError};
use domain::traits::password_hasher::PasswordHasher;
use domain::traits::user_persistence::UserPersistence;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
use tracing::debug;
use validator::Validate;

#[derive(Debug, Error)]
pub enum LoginError {
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Token error: {0}")]
    TokenError(#[from] TokenServiceError),
}

pub type LoginResult<T> = Result<T, LoginError>;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct LoginCommand {
    #[validate(length(min = 1, message = "Username is required"))]
    pub username: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginOutput {
    pub token: String,
    pub refresh_token: String,
    pub user_id: uuid::Uuid,
}

pub struct LoginUseCase {
    user_persistence: Arc<dyn UserPersistence>,
    password_hasher: Arc<dyn PasswordHasher>,
    token_service: Arc<dyn TokenService>,
}

impl LoginUseCase {
    pub fn new(
        user_persistence: Arc<dyn UserPersistence>,
        password_hasher: Arc<dyn PasswordHasher>,
        token_service: Arc<dyn TokenService>,
    ) -> Self {
        Self {
            user_persistence,
            password_hasher,
            token_service,
        }
    }

    pub async fn execute(&self, cmd: LoginCommand) -> LoginResult<LoginOutput> {
        // Validate input
        cmd.validate()
            .map_err(|e| LoginError::Validation(e.to_string()))?;

        // Find user by username
        let user = self
            .user_persistence
            .find_user_by_username(&cmd.username)
            .await
            .map_err(|_| LoginError::InvalidCredentials)?;

        debug!("User found: {:?}", user);

        // Verify password
        if !self
            .password_hasher
            .verify_password(&cmd.password, user.hashed_password())
            .map_err(|_| LoginError::InvalidCredentials)?
        {
            return Err(LoginError::InvalidCredentials);
        }

        // Generate token
        let token = self.token_service.generate_token(&user)?;

        let refresh_token = self.token_service.generate_refresh_token(&user)?;

        Ok(LoginOutput {
            token,
            refresh_token,
            user_id: user.id(),
        })
    }
}
