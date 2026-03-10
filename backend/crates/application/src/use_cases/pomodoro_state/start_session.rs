use std::sync::Arc;

use domain::entities::{
    focus_session::{FocusSession, FocusSessionError, NewSession},
    focus_session_type::FocusSessionType,
};
use thiserror::Error;
use uuid::Uuid;

use crate::repository_traits::{
    focus_session_repository::FocusSessionRepository,
    persistence_error::PersistenceError,
    pomodoro_state_repository::{PomodoroStateRepository, PomodoroStateRepositoryError},
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
}

pub type StartSessionResult<T> = Result<T, StartSessionError>;

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

    pub async fn execute(&self, command: StartSessionCommand) -> StartSessionResult<()> {
        let mut user_state = self
            .pomodoro_state_repo
            .fetch_user_state(command.user_id)
            .await?;

        if let Some(current_session) = user_state.current_session() {
            if current_session.session_type() == FocusSessionType::Work {
                return Err(StartSessionError::WorkSessionAlreadyRunning);
            }

            let terminated_session = current_session.terminate()?;
            self.focus_session_repo
                .create_manual_session(terminated_session)
                .await?;
        }

        let new_session = FocusSession::<NewSession>::new(
            command.user_id,
            user_state.category_id(),
            user_state.task_id(),
            FocusSessionType::Work,
        )?;

        self.pomodoro_state_repo
            .store_running_session(command.user_id, new_session.run_session())
            .await?;

        Ok(())
    }
}
