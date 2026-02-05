use domain::{
    error::persistence_error::PersistenceError, traits::user_persistence::UserPersistence,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum UserInfoError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type UserInfoResult<T> = Result<T, UserInfoError>;

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

    pub async fn execute(&self, user_id: Uuid) -> UserInfoResult<GetUserInfoOutput> {
        let user = self.user_persistence.find_user_by_id(user_id).await?;

        Ok(GetUserInfoOutput {
            id: user.id(),
            username: user.username().to_string(),
            role: format!("{:?}", user.role()),
        })
    }
}

//TODO unit tests
