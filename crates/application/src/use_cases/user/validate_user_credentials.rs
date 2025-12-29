use std::sync::Arc;

use domain::{
    error::persistence_error::PersistenceError,
    traits::{password_hasher::PasswordHasher, user_persistence::UserPersistence},
};
use tracing::info;

use crate::app_error::{AppError, AppResult};

pub struct ValidateUserCredentialsCommand {
    pub username: String,
    pub password: String,
}

pub struct ValidateUserCredentialsUseCase {
    user_persistence: Arc<dyn UserPersistence>,
    password_hasher: Arc<dyn PasswordHasher>,
}

impl ValidateUserCredentialsUseCase {
    pub fn new(
        user_persistence: Arc<dyn UserPersistence>,
        password_hasher: Arc<dyn PasswordHasher>,
    ) -> Self {
        Self {
            user_persistence,
            password_hasher,
        }
    }

    pub async fn execute(&self, cmd: ValidateUserCredentialsCommand) -> AppResult<()> {
        // Validate user params
        if cmd.username.is_empty() || cmd.password.is_empty() {
            return Err(AppError::InvalidUserParam(
                "Username or password is empty".to_string(),
            ));
        }

        // Validate user credentials
        let user = self
            .user_persistence
            .find_user_by_username(&cmd.username)
            .await
            .map_err(|err| match err {
                PersistenceError::NotFound(msg) => {
                    info!("User not found: {}", msg);
                    AppError::InvalidUserParam("Invalid username".to_string())
                }
                _ => AppError::from(err),
            })?;

        if !self
            .password_hasher
            .verify_password(&cmd.password, user.hashed_password())?
        {
            return Err(AppError::InvalidUserParam("Invalid password".to_string()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use domain::entities::{user::User, user_role::UserRole};
    use uuid::Uuid;

    #[test]
    fn test_new_user() {
        let user = User::new(
            "test_user".to_string(),
            "hashed_password".to_string(),
            UserRole::User,
        );

        assert_eq!(user.id().to_string().len(), 36);
        assert_eq!(user.username(), "test_user");
        assert_eq!(user.hashed_password(), "hashed_password");

        assert_eq!(user.role(), &UserRole::User);
        assert!(
            !user.is_admin(),
            "Un nuovo utente User non dovrebbe essere admin"
        );
    }

    #[test]
    fn test_reconstitute_user() {
        let id = Uuid::new_v4();

        let user = User::reconstitute(
            id,
            "admin_user".to_string(),
            "secure_hash".to_string(),
            UserRole::Admin,
        );

        assert_eq!(user.id(), id);
        assert_eq!(user.username(), "admin_user");
        assert_eq!(user.hashed_password(), "secure_hash");

        assert_eq!(user.role(), &UserRole::Admin);
        assert!(
            user.is_admin(),
            "Un utente ricostituito come Admin deve essere admin"
        );
    }

    #[test]
    fn test_update_user_details() {
        let mut user = User::new(
            "old_name".to_string(),
            "old_pass".to_string(),
            UserRole::User,
        );

        user.update_username("new_name".to_string());
        user.update_password("new_pass".to_string());

        assert_eq!(user.username(), "new_name");
        assert_eq!(user.hashed_password(), "new_pass");

        assert_eq!(user.role(), &UserRole::User);
    }

    #[test]
    fn test_update_role() {
        let mut user = User::new("user".to_string(), "pass".to_string(), UserRole::User);

        assert!(!user.is_admin());

        user.update_role(UserRole::Admin);

        assert_eq!(user.role(), &UserRole::Admin);
        assert!(user.is_admin());

        user.update_role(UserRole::User);

        assert_eq!(user.role(), &UserRole::User);
        assert!(!user.is_admin());
    }

    #[test]
    fn test_is_admin_logic() {
        let regular_user = User::new("u".into(), "p".into(), UserRole::User);
        let admin_user = User::new("a".into(), "p".into(), UserRole::Admin);

        assert_eq!(regular_user.is_admin(), false);
        assert_eq!(admin_user.is_admin(), true);
    }
}
