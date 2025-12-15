use std::sync::Arc;

use domain::traits::user_persistence::UserPersistence;
use uuid::Uuid;

use crate::app_error::{AppError, AppResult};

pub struct UpdateUserUsernameCommand {
    pub user_id: Uuid,
    pub new_username: String,
}

pub struct UpdateUserUsernameUseCase {
    user_persistence: Arc<dyn UserPersistence>,
}

impl UpdateUserUsernameUseCase {
    pub fn new(user_persistence: Arc<dyn UserPersistence>) -> Self {
        Self { user_persistence }
    }

    pub async fn execute(&self, cmd: UpdateUserUsernameCommand) -> AppResult<()> {
        // Validate input
        if cmd.new_username.is_empty() {
            return Err(AppError::InvalidUserParam("Invalid username".to_string()));
        }

        // Update user
        let mut user = self.user_persistence.find_user_by_id(cmd.user_id).await?;

        user.update_username(cmd.new_username);

        self.user_persistence.update_user(user).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use domain::error::domain_error::DomainError;
    use domain::{
        entities::{user::User, user_role::UserRole},
        error::persistence_error::PersistenceError,
    };
    use mockall::predicate::eq;
    use uuid::Uuid;

    use crate::{
        app_error::AppError,
        mocks::{MockPasswordHasher, MockPasswordPolicy, MockUserPersistence},
        use_cases::user::update_password::{UpdateUserPasswordCommand, UpdateUserPasswordUseCase},
    };

    #[tokio::test]
    async fn test_update_user_password_success() {
        let mut mock_user_persistence = MockUserPersistence::default();
        let mut password_policy_service = MockPasswordPolicy::default();
        let mut mock_hasher = MockPasswordHasher::default();

        let user_id = Uuid::new_v4();
        let user_id_clone = user_id.clone();

        mock_user_persistence
            .expect_find_user_by_id()
            .with(eq(user_id))
            .times(1)
            .returning(move |_| {
                Ok(User::reconstitute(
                    user_id_clone,
                    "username".to_string(),
                    "hashed_old".to_string(),
                    UserRole::User,
                ))
            });

        mock_user_persistence
            .expect_update_user()
            .times(1)
            .returning(|_| Ok(()));

        password_policy_service
            .expect_validate()
            .with(eq("Password123!"))
            .times(1)
            .returning(|_| Ok(()));

        mock_hasher
            .expect_hash_password()
            .with(eq("old_password"))
            .times(1)
            .returning(|_| Ok("hashed_old".to_string()));

        mock_hasher
            .expect_hash_password()
            .with(eq("Password123!"))
            .times(1)
            .returning(|_| Ok("hashed_new".to_string()));

        let use_case = UpdateUserPasswordUseCase::new(
            Arc::new(mock_hasher),
            Arc::new(mock_user_persistence),
            Arc::new(password_policy_service),
        );

        let cmd = UpdateUserPasswordCommand {
            user_id,
            old_password: "old_password".to_string(),
            new_password: "Password123!".to_string(),
        };

        let result = use_case.execute(cmd).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_user_password_fails_if_old_password_invalid() {
        let mut mock_user_persistence = MockUserPersistence::default();
        let mut password_policy_service = MockPasswordPolicy::default();
        let mut mock_hasher = MockPasswordHasher::default();

        let user_id = Uuid::new_v4();
        let user_id_clone = user_id.clone();

        password_policy_service
            .expect_validate()
            .with(eq("new_password"))
            .times(1)
            .returning(|_| Ok(()));

        mock_user_persistence
            .expect_find_user_by_id()
            .times(1)
            .returning(move |_| {
                Ok(User::reconstitute(
                    user_id_clone,
                    "username".to_string(),
                    "correct_hash".to_string(),
                    UserRole::User,
                ))
            });

        mock_hasher
            .expect_hash_password()
            .with(eq("wrong_old_password"))
            .times(1)
            .returning(|_| Ok("wrong_hash".to_string()));

        mock_user_persistence.expect_update_user().times(0);

        let use_case = UpdateUserPasswordUseCase::new(
            Arc::new(mock_hasher),
            Arc::new(mock_user_persistence),
            Arc::new(password_policy_service),
        );

        let cmd = UpdateUserPasswordCommand {
            user_id,
            old_password: "wrong_old_password".to_string(),
            new_password: "new_password".to_string(),
        };

        let result = use_case.execute(cmd).await;

        assert!(
            matches!(result, Err(AppError::InvalidUserParam(msg)) if msg == "Invalid old password")
        );
    }

    #[tokio::test]
    async fn test_update_user_password_fails_empty_password() {
        let mock_user_persistence = MockUserPersistence::default();
        let password_policy_service = MockPasswordPolicy::default();
        let mock_hasher = MockPasswordHasher::default();

        let use_case = UpdateUserPasswordUseCase::new(
            Arc::new(mock_hasher),
            Arc::new(mock_user_persistence),
            Arc::new(password_policy_service),
        );

        let cmd = UpdateUserPasswordCommand {
            user_id: Uuid::new_v4(),
            old_password: "any".to_string(),
            new_password: "".to_string(),
        };

        let result = use_case.execute(cmd).await;

        assert!(
            matches!(result, Err(AppError::InvalidUserParam(msg)) if msg == "New and old passwords cannot be empty")
        );
    }

    #[tokio::test]
    async fn test_update_user_password_fails_policy_validation() {
        let mock_user_persistence = MockUserPersistence::default();
        let mut password_policy_service = MockPasswordPolicy::default();
        let mock_hasher = MockPasswordHasher::default();

        password_policy_service
            .expect_validate()
            .times(1)
            .returning(|_| Err(DomainError::InvalidPasswordPolicy("Too short".to_string())));

        let use_case = UpdateUserPasswordUseCase::new(
            Arc::new(mock_hasher),
            Arc::new(mock_user_persistence),
            Arc::new(password_policy_service),
        );

        let cmd = UpdateUserPasswordCommand {
            user_id: Uuid::new_v4(),
            old_password: "old".to_string(),
            new_password: "weak".to_string(),
        };

        let result = use_case.execute(cmd).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_user_password_fails_user_not_found() {
        let mut mock_user_persistence = MockUserPersistence::default();
        let mut password_policy_service = MockPasswordPolicy::default();
        let mock_hasher = MockPasswordHasher::default();

        password_policy_service
            .expect_validate()
            .returning(|_| Ok(()));

        mock_user_persistence
            .expect_find_user_by_id()
            .times(1)
            .returning(|_| Err(PersistenceError::NotFound("id".to_string())));

        let use_case = UpdateUserPasswordUseCase::new(
            Arc::new(mock_hasher),
            Arc::new(mock_user_persistence),
            Arc::new(password_policy_service),
        );

        let cmd = UpdateUserPasswordCommand {
            user_id: Uuid::new_v4(),
            old_password: "old".to_string(),
            new_password: "NewPassword1!".to_string(),
        };

        let result = use_case.execute(cmd).await;

        assert!(matches!(result, Err(AppError::UserNotFound(_))));
    }
}
