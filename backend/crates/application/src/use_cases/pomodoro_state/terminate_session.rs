use std::sync::Arc;

use domain::entities::focus_session::FocusSessionError;
use thiserror::Error;
use uuid::Uuid;

use crate::repository_traits::{
    focus_session_repository::FocusSessionRepository,
    persistence_error::PersistenceError,
    pomodoro_state_repository::{PomodoroStateRepository, PomodoroStateRepositoryError},
};

#[derive(Debug, Error, PartialEq)]
pub enum TerminateSessionError {
    #[error("pomodoro state repository error: {0}")]
    PomodoroStateRepositoryError(#[from] PomodoroStateRepositoryError),

    #[error("focus session error: {0}")]
    FocusSessionError(#[from] FocusSessionError),

    #[error("persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type TerminateSessionResult<T> = Result<T, TerminateSessionError>;

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

    pub async fn execute(&self, command: TerminateSessionCommand) -> TerminateSessionResult<()> {
        let current_session = self
            .pomodoro_state_repo
            .fetch_running_session(command.user_id)
            .await?;

        let terminated_session = current_session.terminate()?;
        drop(current_session);

        self.focus_session_repo
            .create_manual_session(terminated_session)
            .await?;

        self.pomodoro_state_repo
            .clear_running_session(command.user_id)
            .await?;

        Ok(())
    }
}
