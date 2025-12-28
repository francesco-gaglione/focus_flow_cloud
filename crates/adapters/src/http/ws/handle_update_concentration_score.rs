use crate::http::{app_state::AppState, ws::update_pomodoro_state::UpdatePomodoroState};
use serde::{Deserialize, Serialize};
use tracing::debug;
use utoipa::ToSchema;
use validator::Validate;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateConcentrationScore {
    #[validate(range(min = 0, max = 5))]
    pub concentration_score: i32,
}

pub async fn handle_update_concentration_score(
    message: &UpdateConcentrationScore,
    state: &AppState,
    user_id: Uuid,
) -> Result<UpdatePomodoroState, String> {
    debug!("Updating concentration score for user {}", user_id);

    let states_map = state.pomodoro_states.read().await;
    let user_state = states_map
        .get(&user_id)
        .ok_or("User state not found".to_string())?
        .clone();
    drop(states_map);

    let mut state = user_state.write().await;

    state.update_current_session_concentration_score(message.concentration_score)?;

    Ok(UpdatePomodoroState::from(state.clone()))
}
