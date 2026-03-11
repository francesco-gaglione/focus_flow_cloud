use std::sync::Arc;
use thiserror::Error;
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
        let mut user_state = self
            .pomodoro_state_repo
            .fetch_user_state(command.user_id)
            .await?;

        if let Some(category_id) = command.category_id {
            user_state.update_category_id(category_id);
        }

        if let Some(task_id) = command.task_id {
            user_state.update_task_id(task_id);
        }

        self.pomodoro_state_repo
            .update_user_state(command.user_id, user_state)
            .await?;

        Ok(())
    }
}
