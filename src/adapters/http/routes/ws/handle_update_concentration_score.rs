use tracing::debug;

use crate::adapters::http::{
    app_state::AppState, dto::ws_msg::update_concentration_score::UpdateConcentrationScore,
};

pub async fn handle_update_concentration_score(
    message: &UpdateConcentrationScore,
    state: &AppState,
) -> Result<(), String> {
    debug!("Updating concentration score");

    let mut state = state.focus_session_state.write().await;

    match &mut state.current_session {
        Some(running_session) => {
            running_session.concentration_score = Some(message.concentration_score);

            Ok(())
        }
        None => Err("No running sessions".to_string()),
    }
}
