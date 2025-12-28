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

        // Retrieve user
        let mut user = self.user_persistence.find_user_by_id(cmd.user_id).await?;

        // Check if username is changing
        if user.username() != cmd.new_username {
            // Check if new username is already taken
            if self
                .user_persistence
                .find_user_by_username(&cmd.new_username)
                .await
                .is_ok()
            {
                return Err(AppError::InvalidUserParam(
                    "Username already exists".to_string(),
                ));
            }

            user.update_username(cmd.new_username);
            self.user_persistence.update_user(user).await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use domain::entities::user::User;
    use domain::entities::user_role::UserRole;
    use domain::error::persistence_error::PersistenceError;
    use mockall::predicate::eq;
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

        // Expect check for uniqueness: returns error (NotFound) meaning it's free
        mock_user_persistence
            .expect_find_user_by_username()
            .with(eq("new_username"))
            .times(1)
            .returning(|_| Err(PersistenceError::NotFound("Not found".to_string())));

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
    async fn test_update_user_username_same_username_no_check() {
        let mut mock_user_persistence = MockUserPersistence::default();
        let user_id = Uuid::new_v4();
        let user_id_clone = user_id.clone();

        mock_user_persistence
            .expect_find_user_by_id()
            .times(1)
            .returning(move |_| {
                Ok(User::reconstitute(
                    user_id_clone,
                    "same_username".to_string(),
                    "hashed_password".to_string(),
                    UserRole::User,
                ))
            });

        // Should NOT call find_user_by_username
        mock_user_persistence
            .expect_find_user_by_username()
            .times(0);

        // Should NOT call update_user
        mock_user_persistence.expect_update_user().times(0);

        let use_case = UpdateUserUsernameUseCase::new(Arc::new(mock_user_persistence));

        let cmd = UpdateUserUsernameCommand {
            user_id: user_id.clone(),
            new_username: "same_username".to_string(),
        };

        let result = use_case.execute(cmd).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_user_username_taken() {
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

        // Username exists
        mock_user_persistence
            .expect_find_user_by_username()
            .with(eq("taken_username"))
            .times(1)
            .returning(|_| {
                Ok(User::new(
                    "taken_username".to_string(),
                    "pwd".to_string(),
                    UserRole::User,
                ))
            });

        let use_case = UpdateUserUsernameUseCase::new(Arc::new(mock_user_persistence));

        let cmd = UpdateUserUsernameCommand {
            user_id: user_id.clone(),
            new_username: "taken_username".to_string(),
        };

        let result = use_case.execute(cmd).await;

        assert!(
            matches!(result, Err(AppError::InvalidUserParam(msg)) if msg == "Username already exists")
        );
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
