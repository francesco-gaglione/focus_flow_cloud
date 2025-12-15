use crate::adapters::http::{
    app_state::AppState,
    ws::update_pomodoro_state::UpdatePomodoroState,
};
use serde::{Deserialize, Serialize};
use tracing::debug;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NoteUpdate {
    pub new_note: String,
}

pub async fn note_update(
    message: &NoteUpdate,
    state: &AppState,
) -> Result<UpdatePomodoroState, String> {
    debug!("Updating current session note");

    let mut pomodoro_state = state.pomodoro_state.write().await;

    pomodoro_state.update_current_session_note(message.new_note.clone())?;

    Ok(UpdatePomodoroState::from(pomodoro_state.clone()))
}
