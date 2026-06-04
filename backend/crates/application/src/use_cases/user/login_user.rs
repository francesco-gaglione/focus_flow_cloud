use crate::auth_traits::password_hasher::PasswordHasher;
use crate::repository_traits::user_persistence::UserPersistence;
use domain::user::services::token_service::{TokenService, TokenServiceError};
use secrecy::{ExposeSecret, SecretBox};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
use tracing::{debug, instrument};

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

pub struct LoginCommand {
    pub username: String,
    pub password: SecretBox<str>,
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

    #[instrument(skip_all)]
    pub async fn execute(&self, cmd: LoginCommand) -> LoginResult<LoginOutput> {
        if cmd.username.is_empty() || cmd.password.expose_secret().is_empty() {
            return Err(LoginError::Validation(
                "Username and password are required".to_string(),
            ));
        }

        let user = self
            .user_persistence
            .find_user_by_username(&cmd.username)
            .await
            .map_err(|_| LoginError::InvalidCredentials)?;

        debug!("User found: {:?}", user);

        if !self
            .password_hasher
            .verify_password(cmd.password.expose_secret(), user.hashed_password())
            .map_err(|_| LoginError::InvalidCredentials)?
        {
            return Err(LoginError::InvalidCredentials);
        }

        let token = self.token_service.generate_token(&user)?;
        let refresh_token = self.token_service.generate_refresh_token(&user)?;

        Ok(LoginOutput {
            token,
            refresh_token,
            user_id: user.id(),
        })
    }
}
