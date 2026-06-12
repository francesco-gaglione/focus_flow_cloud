use std::sync::Arc;

use domain::tasks::entities::{
    focus_session::FocusSessionError, focus_session_type::FocusSessionType,
    pomodoro::pomodoro_state::PomodoroStateError,
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
pub enum StartSessionError {
    #[error("Work session already running")]
    WorkSessionAlreadyRunning,

    #[error("Focus session error: {0}")]
    FocusSessionError(#[from] FocusSessionError),

    #[error("Persistence error: {0}")]
    PersistenceErrror(#[from] PersistenceError),

    #[error("Pomodoro state repository error: {0}")]
    PomodoroStateRepositoryError(#[from] PomodoroStateRepositoryError),

    #[error("Pomodoro state error: {0}")]
    PomodoroStateError(#[from] PomodoroStateError),
}

pub type StartSessionResult<T> = Result<T, StartSessionError>;

#[derive(Debug)]
pub struct StartSessionCommand {
    pub user_id: Uuid,
}

pub struct StartSessionUseCase {
    pomodoro_state_repo: Arc<dyn PomodoroStateRepository>,
    focus_session_repo: Arc<dyn FocusSessionRepository>,
}

impl StartSessionUseCase {
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
    pub async fn execute(&self, command: StartSessionCommand) -> StartSessionResult<()> {
        let mut user_state = self
            .pomodoro_state_repo
            .fetch_user_state(command.user_id)
            .await?;

        if let Some(current_session) = user_state.current_session() {
            if current_session.session_type() == FocusSessionType::Work {
                return Err(StartSessionError::WorkSessionAlreadyRunning);
            }

            let terminated = user_state.terminate_current_session()?;
            self.focus_session_repo
                .create_manual_session(terminated)
                .await?;
        }

        user_state.start_new_session(
            command.user_id,
            FocusSessionType::Work,
            user_state.task_id(),
        )?;

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
    async fn test_start_session_no_existing_session() {
        let mut mock_pomodoro_repo = MockPomodoroStateRepository::new();
        let mock_session_repo = MockFocusSessionRepository::new();
        let user_id = Uuid::new_v4();
        let state = PomodoroState::new();

        mock_pomodoro_repo
            .expect_fetch_user_state()
            .returning(move |_| Ok(state.clone()));
        mock_pomodoro_repo
            .expect_update_user_state()
            .returning(|_, _| Ok(()));

        let use_case =
            StartSessionUseCase::new(Arc::new(mock_pomodoro_repo), Arc::new(mock_session_repo));
        let result = use_case.execute(StartSessionCommand { user_id }).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_start_session_with_break_running() {
        let mut mock_pomodoro_repo = MockPomodoroStateRepository::new();
        let mut mock_session_repo = MockFocusSessionRepository::new();
        let user_id = Uuid::new_v4();
        let mut state = PomodoroState::new();
        state
            .start_new_session(user_id, FocusSessionType::ShortBreak, None)
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
            StartSessionUseCase::new(Arc::new(mock_pomodoro_repo), Arc::new(mock_session_repo));
        let result = use_case.execute(StartSessionCommand { user_id }).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_start_session_work_already_running() {
        let mut mock_pomodoro_repo = MockPomodoroStateRepository::new();
        let mock_session_repo = MockFocusSessionRepository::new();
        let user_id = Uuid::new_v4();
        let mut state = PomodoroState::new();
        state
            .start_new_session(user_id, FocusSessionType::Work, None)
            .unwrap();

        mock_pomodoro_repo
            .expect_fetch_user_state()
            .returning(move |_| Ok(state.clone()));

        let use_case =
            StartSessionUseCase::new(Arc::new(mock_pomodoro_repo), Arc::new(mock_session_repo));
        let result = use_case.execute(StartSessionCommand { user_id }).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            StartSessionError::WorkSessionAlreadyRunning
        ));
    }

    #[tokio::test]
    async fn test_start_session_fetch_repo_error() {
        let mut mock_pomodoro_repo = MockPomodoroStateRepository::new();
        let mock_session_repo = MockFocusSessionRepository::new();

        mock_pomodoro_repo
            .expect_fetch_user_state()
            .returning(|_| Err(PomodoroStateRepositoryError::UserNotFound));

        let use_case =
            StartSessionUseCase::new(Arc::new(mock_pomodoro_repo), Arc::new(mock_session_repo));
        let result = use_case
            .execute(StartSessionCommand {
                user_id: Uuid::new_v4(),
            })
            .await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            StartSessionError::PomodoroStateRepositoryError(_)
        ));
    }
}
