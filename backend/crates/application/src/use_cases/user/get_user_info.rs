use crate::persistence_traits::persistence_error::PersistenceError;
use crate::persistence_traits::user_persistence::UserPersistence;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence_traits::user_persistence::MockUserPersistence;
    use domain::entities::user::User;
    use domain::entities::user_role::UserRole;

    #[tokio::test]
    async fn test_get_user_info_success() {
        let mut mock_persistence = MockUserPersistence::new();
        let user_id = Uuid::new_v4();
        let username = "testuser".to_string();
        let role = UserRole::User;

        let user = User::reconstitute(
            user_id,
            username.clone(),
            "hashed_password".to_string(),
            role.clone(),
        );

        mock_persistence
            .expect_find_user_by_id()
            .with(mockall::predicate::eq(user_id))
            .returning(move |_| Ok(user.clone()));

        let use_case = GetUserInfoUseCase::new(Arc::new(mock_persistence));
        let result = use_case.execute(user_id).await;

        assert!(result.is_ok());
        let info = result.unwrap();
        assert_eq!(info.id, user_id);
        assert_eq!(info.username, username);
        assert_eq!(info.role, format!("{:?}", role));
    }
}
