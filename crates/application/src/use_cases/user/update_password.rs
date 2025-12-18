use domain::error::persistence_error::PersistenceError;
use domain::traits::{
    password_hasher::PasswordHasher, password_policy::PasswordPolicy,
    user_persistence::UserPersistence,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::app_error::{AppError, AppResult};

pub struct UpdateUserPasswordCommand {
    pub user_id: Uuid,
    pub old_password: String,
    pub new_password: String,
}

pub struct UpdateUserPasswordUseCase {
    user_persistence: Arc<dyn UserPersistence>,
    password_hasher: Arc<dyn PasswordHasher>,
    password_policy_service: Arc<dyn PasswordPolicy>,
}

impl UpdateUserPasswordUseCase {
    pub fn new(
        password_hasher: Arc<dyn PasswordHasher>,
        user_persistence: Arc<dyn UserPersistence>,
        password_policy_service: Arc<dyn PasswordPolicy>,
    ) -> Self {
        Self {
            password_hasher,
            user_persistence,
            password_policy_service,
        }
    }

    pub async fn execute(&self, cmd: UpdateUserPasswordCommand) -> AppResult<()> {
        if cmd.new_password.is_empty() || cmd.old_password.is_empty() {
            return Err(AppError::InvalidUserParam(
                "New and old passwords cannot be empty".to_string(),
            ));
        }

        self.password_policy_service.validate(&cmd.new_password)?;

        let mut user = match self.user_persistence.find_user_by_id(cmd.user_id).await {
            Ok(user) => user,
            Err(PersistenceError::NotFound(msg)) => return Err(AppError::UserNotFound(msg)),
            Err(e) => return Err(AppError::from(e)),
        };

        let old_password_hash = self.password_hasher.hash_password(&cmd.old_password)?;

        if old_password_hash != user.hashed_password() {
            return Err(AppError::InvalidUserParam(
                "Invalid old password".to_string(),
            ));
        }

        let hashed_password = self.password_hasher.hash_password(&cmd.new_password)?;

        user.update_password(hashed_password);

        self.user_persistence.update_user(user).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use domain::entities::user::User;
    use domain::entities::user_role::UserRole;
    use uuid::Uuid;

    use crate::{
        app_error::AppError,
        mocks::MockUserPersistence,
        use_cases::user::update_user_username::{
            UpdateUserUsernameCommand, UpdateUserUsernameUseCase,
        },
    };

    #[tokio::test]
    async fn test_update_user_username() {
        let mut mock_user_persistence = MockUserPersistence::default();
        let user_id = Uuid::new_v4();
        let user_id_clone = user_id.clone();

        mock_user_persistence
            .expect_find_user_by_id()
            .times(1)
            .returning(move |_| {
                Ok(User::reconstitute(
                    user_id_clone,
                    "old_username".to_string(),
                    "hashed_password".to_string(),
                    UserRole::User,
                ))
            });

        mock_user_persistence
            .expect_update_user()
            .times(1)
            .returning(|_| Ok(()));

        let use_case = UpdateUserUsernameUseCase::new(Arc::new(mock_user_persistence));

        let cmd = UpdateUserUsernameCommand {
            user_id: user_id.clone(),
            new_username: "new_username".to_string(),
        };

        let result = use_case.execute(cmd).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_user_username_invalid_username() {
        let mut mock_user_persistence = MockUserPersistence::default();
        let user_id = Uuid::new_v4();

        mock_user_persistence.expect_find_user_by_id().times(0);

        mock_user_persistence.expect_update_user().times(0);

        let use_case = UpdateUserUsernameUseCase::new(Arc::new(mock_user_persistence));

        let cmd = UpdateUserUsernameCommand {
            user_id: user_id.clone(),
            new_username: "".to_string(),
        };

        let result = use_case.execute(cmd).await;

        assert!(result.is_err());
        assert_eq!(
            result.err(),
            Some(AppError::InvalidUserParam("Invalid username".to_string()))
        )
    }
}
