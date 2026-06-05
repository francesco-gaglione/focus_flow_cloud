use application::tasks::use_cases::pomodoro_state::{
    fetch_user_pomodoro_state::FetchUserPomodoroStateCommand, start_session::StartSessionCommand,
};
use tracing::debug;
use uuid::Uuid;

use crate::http::{
    app_state::AppState,
    ws::{error::WsHandlerResult, update_pomodoro_state::UpdatePomodoroState},
};

pub async fn handle_start_event(
    state: &AppState,
    user_id: Uuid,
) -> WsHandlerResult<UpdatePomodoroState> {
    debug!("Handling start session event for user {}", user_id);

    let command = StartSessionCommand { user_id };

    state.start_session_uc.execute(command).await?;

    let pomodoro_state = state
        .fetch_pomo_session_uc
        .execute(FetchUserPomodoroStateCommand { user_id })
        .await?;

    Ok(pomodoro_state.into())
}
