use crate::shared::traits::persistence_error::PersistenceError;
use crate::tasks::traits::focus_session_repository::FocusSessionRepository;
use crate::tasks::traits::pomodoro_state_repository::{
    PomodoroStateRepository, PomodoroStateRepositoryError,
};
use domain::tasks::entities::focus_session::FocusSessionError;
use domain::tasks::entities::focus_session_type::FocusSessionType;
use domain::tasks::entities::pomodoro::pomodoro_state::PomodoroStateError;
use std::sync::Arc;
use thiserror::Error;
use tracing::{debug, instrument};
use uuid::Uuid;

#[derive(Debug, Error, PartialEq)]
pub enum PauseSessionError {
    #[error("Session not found")]
    SessionNotFound,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Repository error: {0}")]
    PersistenceError(#[from] PomodoroStateRepositoryError),

    #[error("Failed to pause session: {0}")]
    SessionPauseFailed(String),

    #[error("Failed to create focus session: {0}")]
    FocusSessionError(#[from] FocusSessionError),

    #[error("Failed to persist focus session: {0}")]
    PersistanceError(#[from] PersistenceError),

    #[error("Failed to create manual focus session: {0}")]
    PomodoroStateError(#[from] PomodoroStateError),
}

pub type PauseSessionResult<T> = Result<T, PauseSessionError>;

#[derive(Debug)]
pub struct PauseSessionCommand {
    pub user_id: Uuid,
}

pub struct PauseSessionUseCase {
    pomodoro_state_repo: Arc<dyn PomodoroStateRepository>,
    session_persistence: Arc<dyn FocusSessionRepository>,
}

impl PauseSessionUseCase {
    pub fn new(
        pomodoro_state_repo: Arc<dyn PomodoroStateRepository>,
        focus_session_repo: Arc<dyn FocusSessionRepository>,
    ) -> Self {
        Self {
            pomodoro_state_repo,
            session_persistence: focus_session_repo,
        }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, command: PauseSessionCommand) -> PauseSessionResult<()> {
        let mut user_pomo_state = self
            .pomodoro_state_repo
            .fetch_user_state(command.user_id)
            .await?;

        let current_session = user_pomo_state
            .current_session()
            .ok_or(PauseSessionError::SessionNotFound)?;

        match current_session.session_type() {
            FocusSessionType::Work => {
                debug!("Current session type work, starting break session");
                let terminated_session = user_pomo_state.terminate_current_session()?;

                self.session_persistence
                    .create_manual_session(terminated_session.clone())
                    .await?;

                let next_session_type = user_pomo_state.calculate_next_session_type();
                let task_id = terminated_session.task_id();

                debug!(
                    "next_session_type: {:?}, task_id: {:?}",
                    next_session_type, task_id
                );
                user_pomo_state.start_new_session(command.user_id, next_session_type, task_id)?;

                self.pomodoro_state_repo
                    .update_user_state(command.user_id, user_pomo_state)
                    .await?;
            }
            _ => {
                tracing::error!("Break session already running cannot start a new break");
                return Err(PauseSessionError::SessionPauseFailed(
                    "Pause session already running".to_string(),
                ));
            }
        }

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
    async fn test_pause_work_session_starts_break() {
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
            PauseSessionUseCase::new(Arc::new(mock_pomodoro_repo), Arc::new(mock_session_repo));
        let result = use_case.execute(PauseSessionCommand { user_id }).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_pause_break_session_returns_error() {
        let mut mock_pomodoro_repo = MockPomodoroStateRepository::new();
        let mock_session_repo = MockFocusSessionRepository::new();
        let user_id = Uuid::new_v4();
        let mut state = PomodoroState::new();
        state
            .start_new_session(user_id, FocusSessionType::ShortBreak, None)
            .unwrap();

        mock_pomodoro_repo
            .expect_fetch_user_state()
            .returning(move |_| Ok(state.clone()));

        let use_case =
            PauseSessionUseCase::new(Arc::new(mock_pomodoro_repo), Arc::new(mock_session_repo));
        let result = use_case.execute(PauseSessionCommand { user_id }).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            PauseSessionError::SessionPauseFailed(_)
        ));
    }

    #[tokio::test]
    async fn test_pause_no_running_session() {
        let mut mock_pomodoro_repo = MockPomodoroStateRepository::new();
        let mock_session_repo = MockFocusSessionRepository::new();
        let user_id = Uuid::new_v4();
        let state = PomodoroState::new();

        mock_pomodoro_repo
            .expect_fetch_user_state()
            .returning(move |_| Ok(state.clone()));

        let use_case =
            PauseSessionUseCase::new(Arc::new(mock_pomodoro_repo), Arc::new(mock_session_repo));
        let result = use_case.execute(PauseSessionCommand { user_id }).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            PauseSessionError::SessionNotFound
        ));
    }

    #[tokio::test]
    async fn test_pause_fetch_repo_error() {
        let mut mock_pomodoro_repo = MockPomodoroStateRepository::new();
        let mock_session_repo = MockFocusSessionRepository::new();

        mock_pomodoro_repo
            .expect_fetch_user_state()
            .returning(|_| Err(PomodoroStateRepositoryError::UserNotFound));

        let use_case =
            PauseSessionUseCase::new(Arc::new(mock_pomodoro_repo), Arc::new(mock_session_repo));
        let result = use_case
            .execute(PauseSessionCommand {
                user_id: Uuid::new_v4(),
            })
            .await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            PauseSessionError::PersistenceError(_)
        ));
    }
}
