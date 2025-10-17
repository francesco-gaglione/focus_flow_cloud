use tracing::debug;

use crate::adapters::http::{app_state::AppState, dto::ws_msg::note_update_ws::NoteUpdate};

pub async fn note_update(message: &NoteUpdate, state: &AppState) -> Result<(), String> {
    debug!("Starting session");

    let mut state = state.focus_session_state.write().await;

    match &mut state.current_session {
        Some(running_session) => {
            running_session.note = Some(message.new_note.clone());

            Ok(())
        }
        None => Err("No running sessions".to_string()),
    }
}
