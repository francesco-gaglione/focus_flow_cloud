use crate::repository_traits::pomodoro_state_repository::{
    PomodoroStateRepository, PomodoroStateRepositoryError,
};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum InitPomodoroStateError {
    #[error("Pomodoro repository error: {0}")]
    PomodoroStateRepositoryError(#[from] PomodoroStateRepositoryError),
}

pub type InitPomodoroStateResult<T> = Result<T, InitPomodoroStateError>;

#[derive(Debug)]
pub struct InitPomodoroStateCommand {
    pub user_id: uuid::Uuid,
}

pub struct InitPomodoroStateUseCase {
    pomodoro_state_repo: Arc<dyn PomodoroStateRepository>,
}

impl InitPomodoroStateUseCase {
    pub fn new(pomodoro_state_repo: Arc<dyn PomodoroStateRepository>) -> Self {
        Self {
            pomodoro_state_repo,
        }
    }

    pub async fn execute(&self, command: InitPomodoroStateCommand) -> InitPomodoroStateResult<()> {
        self.pomodoro_state_repo
            .init_user_state(command.user_id)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository_traits::pomodoro_state_repository::MockPomodoroStateRepository;
    use std::sync::Arc;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_init_success() {
        let mut mock_repo = MockPomodoroStateRepository::new();
        mock_repo
            .expect_init_user_state()
            .returning(|_| Ok(()));

        let use_case = InitPomodoroStateUseCase::new(Arc::new(mock_repo));
        let result = use_case
            .execute(InitPomodoroStateCommand { user_id: Uuid::new_v4() })
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_init_repository_error() {
        let mut mock_repo = MockPomodoroStateRepository::new();
        mock_repo
            .expect_init_user_state()
            .returning(|_| Err(PomodoroStateRepositoryError::UserNotFound));

        let use_case = InitPomodoroStateUseCase::new(Arc::new(mock_repo));
        let result = use_case
            .execute(InitPomodoroStateCommand { user_id: Uuid::new_v4() })
            .await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InitPomodoroStateError::PomodoroStateRepositoryError(_)
        ));
    }
}
