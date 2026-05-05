use std::sync::Arc;
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

use crate::repository_traits::pomodoro_state_repository::{
    PomodoroStateRepository, PomodoroStateRepositoryError,
};

#[derive(Debug, Error, PartialEq)]
pub enum UpdatePomodoroContextError {
    #[error("cannot update context while in work session")]
    CannotUpdateContextWhileInWorkSession,

    #[error("pomodoro state repository error: {0}")]
    PomodoroStateRepositoryError(#[from] PomodoroStateRepositoryError),
}

pub type UpdatePomodoroContextResult<T> = Result<T, UpdatePomodoroContextError>;

#[derive(Debug)]
pub struct UpdatePomodoroContextCommand {
    pub user_id: Uuid,
    pub task_id: Option<Uuid>,
}

pub struct UpdatePomodoroContextUseCase {
    pomodoro_state_repo: Arc<dyn PomodoroStateRepository>,
}

impl UpdatePomodoroContextUseCase {
    pub fn new(pomodoro_state_repo: Arc<dyn PomodoroStateRepository>) -> Self {
        Self {
            pomodoro_state_repo,
        }
    }

    #[instrument(skip(self))]
    pub async fn execute(
        &self,
        command: UpdatePomodoroContextCommand,
    ) -> UpdatePomodoroContextResult<()> {
        let mut user_state = self
            .pomodoro_state_repo
            .fetch_user_state(command.user_id)
            .await?;

        if let Some(task_id) = command.task_id {
            user_state.update_task_id(task_id);
        }

        self.pomodoro_state_repo
            .update_user_state(command.user_id, user_state)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository_traits::pomodoro_state_repository::{
        MockPomodoroStateRepository, PomodoroStateRepositoryError,
    };
    use domain::entities::pomodoro::pomodoro_state::PomodoroState;
    use std::sync::Arc;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_update_context_with_category_and_task() {
        let mut mock_repo = MockPomodoroStateRepository::new();
        let user_id = Uuid::new_v4();
        let state = PomodoroState::new();

        mock_repo
            .expect_fetch_user_state()
            .returning(move |_| Ok(state.clone()));
        mock_repo
            .expect_update_user_state()
            .returning(|_, _| Ok(()));

        let use_case = UpdatePomodoroContextUseCase::new(Arc::new(mock_repo));
        let result = use_case
            .execute(UpdatePomodoroContextCommand {
                user_id,
                task_id: Some(Uuid::new_v4()),
            })
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_context_no_fields() {
        let mut mock_repo = MockPomodoroStateRepository::new();
        let user_id = Uuid::new_v4();
        let state = PomodoroState::new();

        mock_repo
            .expect_fetch_user_state()
            .returning(move |_| Ok(state.clone()));
        mock_repo
            .expect_update_user_state()
            .returning(|_, _| Ok(()));

        let use_case = UpdatePomodoroContextUseCase::new(Arc::new(mock_repo));
        let result = use_case
            .execute(UpdatePomodoroContextCommand {
                user_id,
                task_id: None,
            })
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_context_fetch_repo_error() {
        let mut mock_repo = MockPomodoroStateRepository::new();

        mock_repo
            .expect_fetch_user_state()
            .returning(|_| Err(PomodoroStateRepositoryError::UserNotFound));

        let use_case = UpdatePomodoroContextUseCase::new(Arc::new(mock_repo));
        let result = use_case
            .execute(UpdatePomodoroContextCommand {
                user_id: Uuid::new_v4(),
                task_id: None,
            })
            .await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            UpdatePomodoroContextError::PomodoroStateRepositoryError(_)
        ));
    }
}
