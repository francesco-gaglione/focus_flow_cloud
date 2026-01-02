use domain::services::token_service::TokenService;
use domain::traits::user_persistence::UserPersistence;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use validator::Validate;

use crate::app_error::{AppError, AppResult};

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

    pub async fn execute(&self, cmd: RefreshTokenCommand) -> AppResult<RefreshTokenOutput> {
        cmd.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        // Verify refresh token
        let user_id_str = self
            .token_service
            .verify_refresh_token(&cmd.refresh_token)
            .map_err(|_| AppError::Unauthorized("Invalid refresh token".to_string()))?;

        let user_id = uuid::Uuid::parse_str(&user_id_str)
            .map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))?;

        // Find user
        let user = self
            .user_persistence
            .find_user_by_id(user_id)
            .await
            .map_err(|_| AppError::Unauthorized("User not found".to_string()))?;

        // Generate new tokens
        let token = self
            .token_service
            .generate_token(&user)
            .map_err(|e| AppError::GenericError(e.to_string()))?;

        let refresh_token = self
            .token_service
            .generate_refresh_token(&user)
            .map_err(|e| AppError::GenericError(e.to_string()))?;

        Ok(RefreshTokenOutput {
            token,
            refresh_token,
        })
    }
}
