use crate::repository_traits::pomodoro_state_repository::{
    PomodoroStateRepository, PomodoroStateRepositoryError,
};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum InitPomodoroStateError {
    #[error("Pomodoro repository error: {0}")]
    PomodoroStateRepositoryError(#[from] PomodoroStateRepositoryError),
}

pub type InitPomodoroStateResult<T> = Result<T, InitPomodoroStateError>;

#[derive(Debug)]
pub struct InitPomodoroStateCommand {
    pub user_id: uuid::Uuid,
}

pub struct InitPomodoroStateUseCase {
    pomodoro_state_repo: Arc<dyn PomodoroStateRepository>,
}

impl InitPomodoroStateUseCase {
    pub fn new(pomodoro_state_repo: Arc<dyn PomodoroStateRepository>) -> Self {
        Self {
            pomodoro_state_repo,
        }
    }

    pub async fn execute(&self, command: InitPomodoroStateCommand) -> InitPomodoroStateResult<()> {
        self.pomodoro_state_repo
            .init_user_state(command.user_id)
            .await?;

        Ok(())
    }
}
