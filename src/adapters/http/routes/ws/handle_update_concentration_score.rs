use tracing::debug;

use crate::adapters::http::{
    app_state::AppState,
    dto::ws_msg::{
        update_concentration_score::UpdateConcentrationScore,
        update_pomodoro_state::UpdatePomodoroState,
    },
};

pub async fn handle_update_concentration_score(
    message: &UpdateConcentrationScore,
    state: &AppState,
) -> Result<UpdatePomodoroState, String> {
    debug!("Updating concentration score");

    let mut state = state.pomodoro_state.write().await;

    state.update_current_session_concentration_score(message.concentration_score);

    Ok(UpdatePomodoroState::from(state.clone()))
}
