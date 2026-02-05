use std::sync::Arc;

use domain::{
    error::persistence_error::PersistenceError, traits::user_persistence::UserPersistence,
};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum DeleteUserError {
    #[error("Forbidden")]
    Forbidden,

    #[error("Persistence Error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type DeleteUserResult<T> = Result<T, DeleteUserError>;

pub struct DeleteUserCommand {
    pub target_user_id: Uuid,
    pub requester_user_id: Uuid,
}

pub struct DeleteUserUseCase {
    user_persistence: Arc<dyn UserPersistence>,
}

impl DeleteUserUseCase {
    pub fn new(user_persistence: Arc<dyn UserPersistence>) -> Self {
        Self { user_persistence }
    }

    pub async fn execute(&self, cmd: DeleteUserCommand) -> DeleteUserResult<()> {
        // validate permissions
        let requester_is_admin = self
            .user_persistence
            .is_user_admin(cmd.requester_user_id)
            .await?;
        if !requester_is_admin && cmd.target_user_id != cmd.requester_user_id {
            return Err(DeleteUserError::Forbidden);
        }

        // Delete user
        self.user_persistence
            .delete_user(cmd.target_user_id)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use uuid::Uuid;

    use crate::{
        mocks::MockUserPersistence,
        use_cases::user::delete_user::{DeleteUserCommand, DeleteUserUseCase},
    };

    #[tokio::test]
    async fn test_delete_user_by_admin() {
        let mut mock_user_persistence = MockUserPersistence::default();

        let user_id = Uuid::new_v4();
        let requester_id = Uuid::new_v4();

        mock_user_persistence
            .expect_is_user_admin()
            .times(1)
            .returning(|_| Ok(true));

        mock_user_persistence
            .expect_delete_user()
            .times(1)
            .returning(|_| Ok(()));

        let use_case = DeleteUserUseCase::new(Arc::new(mock_user_persistence));
        let result = use_case
            .execute(DeleteUserCommand {
                target_user_id: user_id,
                requester_user_id: requester_id,
            })
            .await;

        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn test_delete_user_by_user() {
        let mut mock_user_persistence = MockUserPersistence::default();

        let user_id = Uuid::new_v4();
        let requester_id = user_id.clone();

        mock_user_persistence
            .expect_is_user_admin()
            .times(1)
            .returning(|_| Ok(false));

        mock_user_persistence
            .expect_delete_user()
            .times(1)
            .returning(|_| Ok(()));

        let use_case = DeleteUserUseCase::new(Arc::new(mock_user_persistence));
        let result = use_case
            .execute(DeleteUserCommand {
                target_user_id: user_id,
                requester_user_id: requester_id,
            })
            .await;

        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn test_delete_user_by_user_with_wrong_id() {
        let mut mock_user_persistence = MockUserPersistence::default();

        let user_id = Uuid::new_v4();
        let requester_id = Uuid::new_v4();

        mock_user_persistence
            .expect_is_user_admin()
            .times(1)
            .returning(|_| Ok(false));

        mock_user_persistence.expect_delete_user().times(0);

        let use_case = DeleteUserUseCase::new(Arc::new(mock_user_persistence));
        let result = use_case
            .execute(DeleteUserCommand {
                target_user_id: user_id,
                requester_user_id: requester_id,
            })
            .await;

        assert!(result.is_err())
    }
}
