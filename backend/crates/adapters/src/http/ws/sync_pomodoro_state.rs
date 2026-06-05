use crate::http::{
    app_state::AppState,
    ws::{error::WsHandlerResult, update_pomodoro_state::UpdatePomodoroState},
};
use application::tasks::use_cases::pomodoro_state::fetch_user_pomodoro_state::FetchUserPomodoroStateCommand;
use uuid::Uuid;

pub async fn sync_pomodoro_state(
    state: &AppState,
    user_id: Uuid,
) -> WsHandlerResult<UpdatePomodoroState> {
    let pomodoro_state = state
        .fetch_pomo_session_uc
        .execute(FetchUserPomodoroStateCommand { user_id })
        .await?;

    Ok(pomodoro_state.into())
}
