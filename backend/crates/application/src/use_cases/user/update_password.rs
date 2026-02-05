use domain::error::persistence_error::PersistenceError;
use domain::traits::password_hasher::HashingError;
use domain::traits::password_policy::PasswordPolicyError;
use domain::traits::{
    password_hasher::PasswordHasher, password_policy::PasswordPolicy,
    user_persistence::UserPersistence,
};
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum UpdatePasswordError {
    #[error("Invalid user parameters")]
    InvalidUserParam(String),

    #[error("Invalid password")]
    InvalidPassword(#[from] PasswordPolicyError),

    #[error("User not found")]
    UserNotFound(String),

    #[error("Persistence error")]
    PersistenceError(#[from] PersistenceError),

    #[error("Hashing error")]
    HashingError(#[from] HashingError),
}

pub type UpdatePasswordResult<T> = Result<T, UpdatePasswordError>;

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

    pub async fn execute(&self, cmd: UpdateUserPasswordCommand) -> UpdatePasswordResult<()> {
        if cmd.new_password.is_empty() || cmd.old_password.is_empty() {
            return Err(UpdatePasswordError::InvalidUserParam(
                "New and old passwords cannot be empty".to_string(),
            ));
        }

        self.password_policy_service.validate(&cmd.new_password)?;

        let mut user = match self.user_persistence.find_user_by_id(cmd.user_id).await {
            Ok(user) => user,
            Err(PersistenceError::NotFound(msg)) => {
                return Err(UpdatePasswordError::UserNotFound(msg))
            }
            Err(e) => return Err(UpdatePasswordError::from(e)),
        };

        if !self
            .password_hasher
            .verify_password(&cmd.old_password, user.hashed_password())?
        {
            return Err(UpdatePasswordError::InvalidUserParam(
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

    use domain::{
        entities::{user::User, user_role::UserRole},
        error::persistence_error::PersistenceError,
        traits::password_policy::PasswordPolicyError,
    };
    use mockall::predicate::eq;
    use uuid::Uuid;

    use crate::{
        mocks::{MockPasswordHasher, MockPasswordPolicy, MockUserPersistence},
        use_cases::user::update_password::{
            UpdatePasswordError, UpdateUserPasswordCommand, UpdateUserPasswordUseCase,
        },
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
            .expect_verify_password()
            .with(eq("old_password"), eq("hashed_old"))
            .times(1)
            .returning(|_, _| Ok(true));

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
            .expect_verify_password()
            .with(eq("wrong_old_password"), eq("correct_hash"))
            .times(1)
            .returning(|_, _| Ok(false));

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
            matches!(result, Err(UpdatePasswordError::InvalidUserParam(msg)) if msg == "Invalid old password")
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
            matches!(result, Err(UpdatePasswordError::InvalidUserParam(msg)) if msg == "New and old passwords cannot be empty")
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
            .returning(|_| Err(PasswordPolicyError::InvalidLength("Too short".to_string())));

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

        assert!(matches!(
            result,
            Err(UpdatePasswordError::InvalidPassword(_))
        ));
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

        assert!(matches!(result, Err(UpdatePasswordError::UserNotFound(_))));
    }
}
