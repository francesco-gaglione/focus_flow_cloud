use domain::traits::user_persistence::UserPersistence;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::app_error::{AppError, AppResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserInfoOutput {
    pub id: Uuid,
    pub username: String,
    pub role: String,
}

pub struct GetUserInfoUseCase {
    user_persistence: Arc<dyn UserPersistence>,
}

impl GetUserInfoUseCase {
    pub fn new(user_persistence: Arc<dyn UserPersistence>) -> Self {
        Self { user_persistence }
    }

    pub async fn execute(&self, user_id: Uuid) -> AppResult<GetUserInfoOutput> {
        let user = self
            .user_persistence
            .find_user_by_id(user_id)
            .await
            .map_err(|_| AppError::UserNotFound("User not found".to_string()))?;

        Ok(GetUserInfoOutput {
            id: user.id(),
            username: user.username().to_string(),
            role: format!("{:?}", user.role()),
        })
    }
}
