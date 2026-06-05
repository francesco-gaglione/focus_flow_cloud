use async_trait::async_trait;
use domain::tasks::entities::pomodoro::pomodoro_state::PomodoroState;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Error, PartialEq)]
pub enum PomodoroStateRepositoryError {
    #[error("session not found")]
    SessionNotFound,

    #[error("already running")]
    AlreadyRunning,

    #[error("user not found")]
    UserNotFound,
}

pub type PomodoroStateResult<T> = Result<T, PomodoroStateRepositoryError>;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PomodoroStateRepository: Send + Sync {
    async fn init_user_state(&self, user_id: Uuid) -> PomodoroStateResult<()>;

    async fn fetch_user_state(&self, user_id: Uuid) -> PomodoroStateResult<PomodoroState>;

    async fn update_user_state(
        &self,
        user_id: Uuid,
        state: PomodoroState,
    ) -> PomodoroStateResult<()>;

    async fn clear_user_state(&self, user_id: Uuid) -> PomodoroStateResult<()>;
}
