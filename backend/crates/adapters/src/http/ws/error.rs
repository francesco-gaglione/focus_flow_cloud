use application::use_cases::pomodoro_state::{
    fetch_user_pomodoro_state::FetchUserPomodoroStateError, pause_session::PauseSessionError,
    start_session::StartSessionError, terminate_session::TerminateSessionError,
    update_current_session::UpdateSessionError,
    update_pomodoro_context::UpdatePomodoroContextError,
};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum WsHandlerError {
    #[error("fetch user pomodoro state error: {0}")]
    FetchUserPomodoroStateError(#[from] FetchUserPomodoroStateError),

    #[error("pause session error: {0}")]
    PauseSessionError(#[from] PauseSessionError),

    #[error("update pomodoro context error: {0}")]
    UpdatePomodoroContextError(#[from] UpdatePomodoroContextError),

    #[error("start session error: {0}")]
    StartSessionError(#[from] StartSessionError),

    #[error("terminate session error: {0}")]
    TerminateSessionError(#[from] TerminateSessionError),

    #[error("update session error: {0}")]
    UpdateSessionError(#[from] UpdateSessionError),

    #[error("validation error: {0}")]
    ValidationError(String),

    #[error("bad request: {0}")]
    BadRequest(String),
}

pub type WsHandlerResult<T> = Result<T, WsHandlerError>;
