use std::sync::Arc;

use domain::{
    entities::{user::User, user_role::UserRole},
    error::persistence_error::PersistenceError,
    traits::{
        password_hasher::{HashingError, PasswordHasher},
        password_policy::{PasswordPolicy, PasswordPolicyError},
        user_persistence::UserPersistence,
    },
};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum RegisterUserError {
    #[error("Invalid user credentials: {0}")]
    InvalidUserCredentials(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Password policy violation")]
    PasswordPolicyViolation(#[from] PasswordPolicyError),

    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),

    #[error("Password error: {0}")]
    PasswordError(#[from] HashingError),
}

pub type RegisterUserResult<T> = Result<T, RegisterUserError>;

pub struct RegisterUserCommand {
    pub username: String,
    pub password: String,
    pub requester_user_id: Uuid,
}

pub struct RegisterUserUseCase {
    password_hasher: Arc<dyn PasswordHasher>,
    user_persistence: Arc<dyn UserPersistence>,
    password_policy_service: Arc<dyn PasswordPolicy>,
}

impl RegisterUserUseCase {
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

    pub async fn execute(&self, cmd: RegisterUserCommand) -> RegisterUserResult<Uuid> {
        // Validate input
        if cmd.username.is_empty() || cmd.password.is_empty() {
            return Err(RegisterUserError::InvalidUserCredentials(
                "Username and/or password cannot be empty".to_string(),
            ));
        }

        self.password_policy_service.validate(&cmd.password)?;

        // Check if current user is admin
        let requester_user = self
            .user_persistence
            .find_user_by_id(cmd.requester_user_id)
            .await?;

        if !requester_user.is_admin() {
            return Err(RegisterUserError::Unauthorized(
                "Only admin users can register new users".to_string(),
            ));
        }

        // Register user
        let hashed_password = self.password_hasher.hash_password(&cmd.password)?;
        let user = User::new(cmd.username, hashed_password, UserRole::User);
        let user_id = self.user_persistence.create_user(user).await?;
        Ok(user_id)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use domain::entities::{user::User, user_role::UserRole};
    use domain::error::persistence_error::PersistenceError;
    use domain::traits::password_policy::PasswordPolicyError;
    use uuid::Uuid;

    use crate::{
        mocks::{MockPasswordHasher, MockPasswordPolicy, MockUserPersistence},
        use_cases::user::register_user::{
            RegisterUserCommand, RegisterUserError, RegisterUserUseCase,
        },
    };

    #[tokio::test]
    async fn test_register_user() {
        let mut mock_user_persistence = MockUserPersistence::default();
        let mut password_policy_service = MockPasswordPolicy::default();
        let mut mock_hasher = MockPasswordHasher::default();

        password_policy_service
            .expect_validate()
            .times(1)
            .returning(|_| Ok(()));

        mock_user_persistence
            .expect_find_user_by_id()
            .times(1)
            .returning(|_| {
                let user = User::new("admin".to_string(), "hash".to_string(), UserRole::Admin);
                Ok(user)
            });

        mock_hasher
            .expect_hash_password()
            .times(1)
            .returning(|_| Ok("hashed_password".to_string()));

        mock_user_persistence
            .expect_create_user()
            .times(1)
            .returning(|_| Ok(Uuid::new_v4()));

        let use_case = RegisterUserUseCase::new(
            Arc::new(mock_hasher),
            Arc::new(mock_user_persistence),
            Arc::new(password_policy_service),
        );

        let cmd = RegisterUserCommand {
            username: "test_user".to_string(),
            password: "Password1".to_string(),
            requester_user_id: Uuid::new_v4(),
        };

        let result = use_case.execute(cmd).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_register_user_with_invalid_password() {
        let mut mock_user_persistence = MockUserPersistence::default();
        let mut password_policy_service = MockPasswordPolicy::default();
        let mut mock_hasher = MockPasswordHasher::default();

        password_policy_service
            .expect_validate()
            .times(1)
            .returning(|_| {
                Err(PasswordPolicyError::InvalidLength(
                    "Invalid password".to_string(),
                ))
            });

        mock_hasher.expect_hash_password().times(0);

        mock_user_persistence.expect_create_user().times(0);

        let use_case = RegisterUserUseCase::new(
            Arc::new(mock_hasher),
            Arc::new(mock_user_persistence),
            Arc::new(password_policy_service),
        );

        let cmd = RegisterUserCommand {
            username: "test_user".to_string(),
            password: "Password1".to_string(),
            requester_user_id: Uuid::new_v4(),
        };

        let result = use_case.execute(cmd).await;

        assert!(result.is_err());
        assert!(matches!(
            result.err(),
            Some(RegisterUserError::PasswordPolicyViolation(_))
        ));
    }

    #[tokio::test]
    async fn test_register_user_with_empty_password() {
        let mut mock_user_persistence = MockUserPersistence::default();
        let mut password_policy_service = MockPasswordPolicy::default();
        let mut mock_hasher = MockPasswordHasher::default();

        password_policy_service.expect_validate().times(0);
        mock_hasher.expect_hash_password().times(0);
        mock_user_persistence.expect_create_user().times(0);

        let use_case = RegisterUserUseCase::new(
            Arc::new(mock_hasher),
            Arc::new(mock_user_persistence),
            Arc::new(password_policy_service),
        );

        let cmd = RegisterUserCommand {
            username: "test_user".to_string(),
            password: "".to_string(),
            requester_user_id: Uuid::new_v4(),
        };

        let result = use_case.execute(cmd).await;

        assert!(result.is_err());
        assert!(matches!(
            result.err(),
            Some(RegisterUserError::InvalidUserCredentials(_))
        ));
    }

    #[tokio::test]
    async fn test_register_user_with_empty_username() {
        let mut mock_user_persistence = MockUserPersistence::default();
        let mut password_policy_service = MockPasswordPolicy::default();
        let mut mock_hasher = MockPasswordHasher::default();

        password_policy_service.expect_validate().times(0);
        mock_hasher.expect_hash_password().times(0);
        mock_user_persistence.expect_create_user().times(0);

        let use_case = RegisterUserUseCase::new(
            Arc::new(mock_hasher),
            Arc::new(mock_user_persistence),
            Arc::new(password_policy_service),
        );

        let cmd = RegisterUserCommand {
            username: "".to_string(),
            password: "password".to_string(),
            requester_user_id: Uuid::new_v4(),
        };

        let result = use_case.execute(cmd).await;

        assert!(result.is_err());
        assert!(matches!(
            result.err(),
            Some(RegisterUserError::InvalidUserCredentials(_))
        ));
    }

    #[tokio::test]
    async fn test_register_user_fails_persistence_error() {
        let mut mock_user_persistence = MockUserPersistence::default();
        let mut password_policy_service = MockPasswordPolicy::default();
        let mut mock_hasher = MockPasswordHasher::default();

        password_policy_service
            .expect_validate()
            .returning(|_| Ok(()));

        mock_user_persistence
            .expect_find_user_by_id()
            .times(1)
            .returning(|_| {
                let user = User::new("admin".to_string(), "hash".to_string(), UserRole::Admin);
                Ok(user)
            });

        mock_hasher
            .expect_hash_password()
            .returning(|_| Ok("hashed".to_string()));

        mock_user_persistence
            .expect_create_user()
            .times(1)
            .returning(|_| Err(PersistenceError::Unexpected("DB Error".to_string())));

        let use_case = RegisterUserUseCase::new(
            Arc::new(mock_hasher),
            Arc::new(mock_user_persistence),
            Arc::new(password_policy_service),
        );

        let cmd = RegisterUserCommand {
            username: "user".to_string(),
            password: "Password1".to_string(),
            requester_user_id: Uuid::new_v4(),
        };

        let result = use_case.execute(cmd).await;

        println!("{:?}", result);

        assert!(matches!(
            result,
            Err(RegisterUserError::PersistenceError(_))
        ));
    }

    #[tokio::test]
    async fn test_register_user_fails_if_requester_is_not_admin() {
        let mut mock_user_persistence = MockUserPersistence::default();
        let mut password_policy_service = MockPasswordPolicy::default();
        let mut mock_hasher = MockPasswordHasher::default();

        password_policy_service
            .expect_validate()
            .returning(|_| Ok(()));

        mock_user_persistence
            .expect_find_user_by_id()
            .times(1)
            .returning(|_| {
                let user = User::new(
                    "regular_user".to_string(),
                    "hash".to_string(),
                    UserRole::User,
                );
                Ok(user)
            });

        // Should not be called
        mock_hasher.expect_hash_password().times(0);
        mock_user_persistence.expect_create_user().times(0);

        let use_case = RegisterUserUseCase::new(
            Arc::new(mock_hasher),
            Arc::new(mock_user_persistence),
            Arc::new(password_policy_service),
        );

        let cmd = RegisterUserCommand {
            username: "new_user".to_string(),
            password: "Password1".to_string(),
            requester_user_id: Uuid::new_v4(),
        };

        let result = use_case.execute(cmd).await;

        assert!(result.is_err());
        match result.err() {
            Some(RegisterUserError::Unauthorized(msg)) => {
                assert_eq!(msg, "Only admin users can register new users");
            }
            _ => panic!("Expected Unauthorized error"),
        }
    }
}
