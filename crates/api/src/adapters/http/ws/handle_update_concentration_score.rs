use crate::adapters::http::{app_state::AppState, ws::update_pomodoro_state::UpdatePomodoroState};
use serde::{Deserialize, Serialize};
use tracing::debug;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateConcentrationScore {
    #[validate(range(min = 0, max = 5))]
    pub concentration_score: i32,
}

pub async fn handle_update_concentration_score(
    message: &UpdateConcentrationScore,
    state: &AppState,
) -> Result<UpdatePomodoroState, String> {
    debug!("Updating concentration score");

    let mut state = state.pomodoro_state.write().await;

    state.update_current_session_concentration_score(message.concentration_score)?;

    Ok(UpdatePomodoroState::from(state.clone()))
}
