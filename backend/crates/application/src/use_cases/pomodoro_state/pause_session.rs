use crate::repository_traits::focus_session_repository::FocusSessionRepository;
use crate::repository_traits::persistence_error::PersistenceError;
use crate::repository_traits::pomodoro_state_repository::{
    PomodoroStateRepository, PomodoroStateRepositoryError,
};
use domain::entities::focus_session::FocusSessionError;
use domain::entities::focus_session_type::FocusSessionType;
use domain::entities::pomodoro::pomodoro_state::PomodoroStateError;
use std::sync::Arc;
use thiserror::Error;
use tracing::debug;
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
                let category_id = terminated_session.category_id();
                let task_id = terminated_session.task_id();

                debug!(
                    "next_session_type: {:?}, category_id: {:?}, task_id: {:?}",
                    next_session_type, category_id, task_id
                );
                user_pomo_state.start_new_session(
                    command.user_id,
                    next_session_type,
                    category_id,
                    task_id,
                )?;

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
