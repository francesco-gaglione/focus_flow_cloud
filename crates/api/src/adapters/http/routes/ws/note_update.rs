use tracing::debug;

use crate::adapters::http::{
    app_state::AppState,
    dto::ws_msg::{note_update_ws::NoteUpdate, update_pomodoro_state::UpdatePomodoroState},
};

pub async fn note_update(
    message: &NoteUpdate,
    state: &AppState,
) -> Result<UpdatePomodoroState, String> {
    debug!("Updating current session note");

    let mut pomodoro_state = state.pomodoro_state.write().await;

    pomodoro_state.update_current_session_note(message.new_note.clone())?;

    Ok(UpdatePomodoroState::from(pomodoro_state.clone()))
}
