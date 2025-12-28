use crate::http::{app_state::AppState, ws::update_pomodoro_state::UpdatePomodoroState};
use serde::{Deserialize, Serialize};
use tracing::debug;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NoteUpdate {
    pub new_note: String,
}

pub async fn note_update(
    message: &NoteUpdate,
    state: &AppState,
    user_id: Uuid,
) -> Result<UpdatePomodoroState, String> {
    debug!("Updating current session note for user {}", user_id);

    let states_map = state.pomodoro_states.read().await;
    let user_state = states_map
        .get(&user_id)
        .ok_or("User state not found".to_string())?
        .clone();
    drop(states_map);

    let mut pomodoro_state = user_state.write().await;

    pomodoro_state.update_current_session_note(message.new_note.clone())?;

    Ok(UpdatePomodoroState::from(pomodoro_state.clone()))
}
