use std::sync::Arc;

use domain::entities::focus_session::FocusSessionError;
use thiserror::Error;
use uuid::Uuid;

use crate::repository_traits::{
    persistence_error::PersistenceError,
    pomodoro_state_repository::{PomodoroStateRepository, PomodoroStateRepositoryError},
};

#[derive(Debug, Error, PartialEq)]
pub enum UpdateSessionError {
    #[error("pomodoro state repository error: {0}")]
    PomodoroStateRepositoryError(#[from] PomodoroStateRepositoryError),

    #[error("focus session error: {0}")]
    FocusSessionError(#[from] FocusSessionError),

    #[error("persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),

    #[error("no current session found")]
    NoCurrentSession,
}

pub type UpdateSessionResult<T> = Result<T, UpdateSessionError>;

pub struct UpdateSessionCommand {
    pub user_id: Uuid,
    pub new_note: Option<String>,
    pub new_concentration_score: Option<i32>,
}

pub struct UpdateSessionUseCase {
    pomodoro_state_repo: Arc<dyn PomodoroStateRepository>,
}

impl UpdateSessionUseCase {
    pub fn new(pomodoro_state_repo: Arc<dyn PomodoroStateRepository>) -> Self {
        Self {
            pomodoro_state_repo,
        }
    }

    pub async fn execute(&self, command: UpdateSessionCommand) -> UpdateSessionResult<()> {
        let mut user_state = self
            .pomodoro_state_repo
            .fetch_user_state(command.user_id)
            .await?;

        let mut current_session = user_state
            .current_session()
            .ok_or_else(|| UpdateSessionError::NoCurrentSession)?;

        if let Some(note) = command.new_note {
            current_session.update_note(note);
        }
        if let Some(concentration_score) = command.new_concentration_score {
            current_session.update_concentration_score(concentration_score)?;
        }

        user_state.update_current_session(current_session);

        self.pomodoro_state_repo
            .update_user_state(command.user_id, user_state)
            .await?;

        Ok(())
    }
}
