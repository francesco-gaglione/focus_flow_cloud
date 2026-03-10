use crate::http::ws::error::WsHandlerResult;
use crate::http::{app_state::AppState, ws::update_pomodoro_state::UpdatePomodoroState};
use application::use_cases::pomodoro_state::fetch_user_pomodoro_state::FetchUserPomodoroStateCommand;
use application::use_cases::pomodoro_state::pause_session::PauseSessionCommand;
use tracing::debug;
use uuid::Uuid;

pub async fn handle_break_event(
    state: &AppState,
    user_id: Uuid,
) -> WsHandlerResult<UpdatePomodoroState> {
    debug!("Handling break session event for user {}", user_id);

    let command = PauseSessionCommand { user_id };

    state.pause_pomo_session_uc.execute(command).await?;

    let pomodoro_state = state
        .fetch_pomo_session_uc
        .execute(FetchUserPomodoroStateCommand { user_id })
        .await?;

    Ok(pomodoro_state.into())
}
