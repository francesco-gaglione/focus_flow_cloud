use domain::services::token_service::TokenService;
use domain::traits::password_hasher::PasswordHasher;
use domain::traits::user_persistence::UserPersistence;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::debug;
use validator::Validate;

use crate::app_error::{AppError, AppResult};

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct LoginCommand {
    #[validate(length(min = 1, message = "Username is required"))]
    pub username: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
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

    pub async fn execute(&self, cmd: LoginCommand) -> AppResult<String> {
        // Validate input
        cmd.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        // Find user by username
        let user = self
            .user_persistence
            .find_user_by_username(&cmd.username)
            .await
            .map_err(|_| AppError::Unauthorized("Invalid credentials".to_string()))?;

        debug!("User found: {:?}", user);

        // Verify password
        if !self
            .password_hasher
            .verify_password(&cmd.password, user.hashed_password())
            .map_err(|_| AppError::Unauthorized("Invalid credentials".to_string()))?
        {
            return Err(AppError::Unauthorized("Invalid credentials".to_string()));
        }

        // Generate token
        let token = self
            .token_service
            .generate_token(&user)
            .map_err(|e| AppError::GenericError(e.to_string()))?;

        Ok(token)
    }
}
