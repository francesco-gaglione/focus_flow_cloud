use std::sync::Arc;

use domain::entities::focus_session_type::FocusSessionType;
use thiserror::Error;
use tracing::error;
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

pub struct UpdatePomodoroContextCommand {
    pub user_id: Uuid,
    pub category_id: Option<Uuid>,
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

    pub async fn execute(
        &self,
        command: UpdatePomodoroContextCommand,
    ) -> UpdatePomodoroContextResult<()> {
        let running_session = self
            .pomodoro_state_repo
            .fetch_running_session(command.user_id)
            .await?;

        if running_session.session_type() == FocusSessionType::Work {
            error!("cannot update context while in work session");
            return Err(UpdatePomodoroContextError::CannotUpdateContextWhileInWorkSession);
        }

        //TODO check if task_id is valid for the given category_id

        self.pomodoro_state_repo
            .update_work_context(command.user_id, command.category_id, command.task_id)
            .await?;

        Ok(())
    }
}
