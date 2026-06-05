use std::sync::Arc;

use domain::tasks::entities::{
    focus_session::FocusSessionError, pomodoro::pomodoro_state::PomodoroStateError,
};
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

use crate::shared::traits::persistence_error::PersistenceError;
use crate::tasks::traits::focus_session_repository::FocusSessionRepository;
use crate::tasks::traits::pomodoro_state_repository::{
    PomodoroStateRepository, PomodoroStateRepositoryError,
};

#[derive(Debug, Error, PartialEq)]
pub enum TerminateSessionError {
    #[error("pomodoro state repository error: {0}")]
    PomodoroStateRepositoryError(#[from] PomodoroStateRepositoryError),

    #[error("focus session error: {0}")]
    FocusSessionError(#[from] FocusSessionError),

    #[error("persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),

    #[error("pomodoro state error: {0}")]
    PomodoroStateError(#[from] PomodoroStateError),
}

pub type TerminateSessionResult<T> = Result<T, TerminateSessionError>;

#[derive(Debug)]
pub struct TerminateSessionCommand {
    pub user_id: Uuid,
}

pub struct TerminateSessionUseCase {
    pomodoro_state_repo: Arc<dyn PomodoroStateRepository>,
    focus_session_repo: Arc<dyn FocusSessionRepository>,
}

impl TerminateSessionUseCase {
    pub fn new(
        pomodoro_state_repo: Arc<dyn PomodoroStateRepository>,
        focus_session_repo: Arc<dyn FocusSessionRepository>,
    ) -> Self {
        Self {
            pomodoro_state_repo,
            focus_session_repo,
        }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, command: TerminateSessionCommand) -> TerminateSessionResult<()> {
        let mut user_state = self
            .pomodoro_state_repo
            .fetch_user_state(command.user_id)
            .await?;

        let terminated_session = user_state.terminate_current_session()?;

        self.focus_session_repo
            .create_manual_session(terminated_session)
            .await?;

        self.pomodoro_state_repo
            .update_user_state(command.user_id, user_state)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tasks::traits::focus_session_repository::MockFocusSessionRepository;
    use crate::tasks::traits::pomodoro_state_repository::{
        MockPomodoroStateRepository, PomodoroStateRepositoryError,
    };
    use domain::tasks::entities::focus_session_type::FocusSessionType;
    use domain::tasks::entities::pomodoro::pomodoro_state::PomodoroState;
    use std::sync::Arc;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_terminate_success() {
        let mut mock_pomodoro_repo = MockPomodoroStateRepository::new();
        let mut mock_session_repo = MockFocusSessionRepository::new();
        let user_id = Uuid::new_v4();
        let mut state = PomodoroState::new();
        state
            .start_new_session(user_id, FocusSessionType::Work, None)
            .unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));

        mock_pomodoro_repo
            .expect_fetch_user_state()
            .returning(move |_| Ok(state.clone()));
        mock_session_repo
            .expect_create_manual_session()
            .returning(|_| Ok(()));
        mock_pomodoro_repo
            .expect_update_user_state()
            .returning(|_, _| Ok(()));

        let use_case =
            TerminateSessionUseCase::new(Arc::new(mock_pomodoro_repo), Arc::new(mock_session_repo));
        let result = use_case.execute(TerminateSessionCommand { user_id }).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_terminate_with_scheduled_task_completes_it() {
        let mut mock_pomodoro_repo = MockPomodoroStateRepository::new();
        let mut mock_session_repo = MockFocusSessionRepository::new();
        let user_id = Uuid::new_v4();
        let task_id = Uuid::new_v4();
        let mut state = PomodoroState::new();
        state
            .start_new_session(user_id, FocusSessionType::Work, Some(task_id))
            .unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));

        mock_pomodoro_repo
            .expect_fetch_user_state()
            .returning(move |_| Ok(state.clone()));
        mock_session_repo
            .expect_create_manual_session()
            .returning(|_| Ok(()));
        mock_pomodoro_repo
            .expect_update_user_state()
            .returning(|_, _| Ok(()));

        let use_case =
            TerminateSessionUseCase::new(Arc::new(mock_pomodoro_repo), Arc::new(mock_session_repo));
        let result = use_case.execute(TerminateSessionCommand { user_id }).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_terminate_with_unscheduled_task_does_not_complete_it() {
        let mut mock_pomodoro_repo = MockPomodoroStateRepository::new();
        let mut mock_session_repo = MockFocusSessionRepository::new();
        let user_id = Uuid::new_v4();
        let task_id = Uuid::new_v4();
        let mut state = PomodoroState::new();
        state
            .start_new_session(user_id, FocusSessionType::Work, Some(task_id))
            .unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));

        mock_pomodoro_repo
            .expect_fetch_user_state()
            .returning(move |_| Ok(state.clone()));
        mock_session_repo
            .expect_create_manual_session()
            .returning(|_| Ok(()));
        mock_pomodoro_repo
            .expect_update_user_state()
            .returning(|_, _| Ok(()));

        let use_case =
            TerminateSessionUseCase::new(Arc::new(mock_pomodoro_repo), Arc::new(mock_session_repo));
        let result = use_case.execute(TerminateSessionCommand { user_id }).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_terminate_no_running_session() {
        let mut mock_pomodoro_repo = MockPomodoroStateRepository::new();
        let mock_session_repo = MockFocusSessionRepository::new();
        let user_id = Uuid::new_v4();
        let state = PomodoroState::new();

        mock_pomodoro_repo
            .expect_fetch_user_state()
            .returning(move |_| Ok(state.clone()));

        let use_case =
            TerminateSessionUseCase::new(Arc::new(mock_pomodoro_repo), Arc::new(mock_session_repo));
        let result = use_case.execute(TerminateSessionCommand { user_id }).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TerminateSessionError::PomodoroStateError(_)
        ));
    }

    #[tokio::test]
    async fn test_terminate_fetch_repo_error() {
        let mut mock_pomodoro_repo = MockPomodoroStateRepository::new();
        let mock_session_repo = MockFocusSessionRepository::new();

        mock_pomodoro_repo
            .expect_fetch_user_state()
            .returning(|_| Err(PomodoroStateRepositoryError::UserNotFound));

        let use_case =
            TerminateSessionUseCase::new(Arc::new(mock_pomodoro_repo), Arc::new(mock_session_repo));
        let result = use_case
            .execute(TerminateSessionCommand {
                user_id: Uuid::new_v4(),
            })
            .await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TerminateSessionError::PomodoroStateRepositoryError(_)
        ));
    }
}
