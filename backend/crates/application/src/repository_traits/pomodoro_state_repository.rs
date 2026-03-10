use async_trait::async_trait;
use domain::entities::focus_session::{FocusSession, RunningSession};
use domain::entities::pomodoro::pomodoro_state::PomodoroState;
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
    async fn fetch_user_state(&self, user_id: Uuid) -> PomodoroStateResult<PomodoroState>;

    async fn update_work_context(
        &self,
        user_id: Uuid,
        category_id: Option<Uuid>,
        task_id: Option<Uuid>,
    ) -> PomodoroStateResult<()>;

    async fn store_running_session(
        &self,
        user_id: Uuid,
        session: FocusSession<RunningSession>,
    ) -> PomodoroStateResult<()>;

    async fn update_running_session(
        &self,
        user_id: Uuid,
        session: FocusSession<RunningSession>,
    ) -> PomodoroStateResult<()>;

    async fn fetch_running_session(
        &self,
        user_id: Uuid,
    ) -> PomodoroStateResult<FocusSession<RunningSession>>;

    async fn clear_running_session(&self, user_id: Uuid) -> PomodoroStateResult<()>;
}
