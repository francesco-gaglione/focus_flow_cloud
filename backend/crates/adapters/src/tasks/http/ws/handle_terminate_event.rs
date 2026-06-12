use application::tasks::use_cases::pomodoro_state::{
    fetch_user_pomodoro_state::FetchUserPomodoroStateCommand,
    terminate_session::TerminateSessionCommand,
};
use tracing::debug;
use uuid::Uuid;

use crate::shared::http::app_state::AppState;
use crate::tasks::http::ws::error::WsHandlerResult;
use crate::tasks::http::ws::update_pomodoro_state::UpdatePomodoroState;

pub async fn handle_terminate_event(
    state: &AppState,
    user_id: Uuid,
) -> WsHandlerResult<UpdatePomodoroState> {
    debug!("Handling terminate event for user {}", user_id);

    let command = TerminateSessionCommand { user_id };

    state.tasks.terminate_session_uc.execute(command).await?;

    let pomodoro_state = state
        .tasks.fetch_pomo_session_uc
        .execute(FetchUserPomodoroStateCommand { user_id })
        .await?;

    Ok(pomodoro_state.into())
}
