use tracing::debug;

use crate::adapters::http::app_state::AppState;

/// Resets the entire Pomodoro session state.
/// This should be called when the user finishes their work session
/// (e.g., before shutting down the computer).
/// Requires all active sessions to be completed first.
pub async fn end_session(state: &AppState) -> Result<(), String> {
    debug!("Ending all sessions and resetting state");

    let mut session_state = state.focus_session_state.write().await;

    match &session_state.current_session {
        Some(_) => Err("Cannot end session: active session must be completed first".to_string()),
        None => {
            // Reset entire state
            session_state.consecutive_sessions.clear();
            session_state.workspace.category_id = None;
            session_state.workspace.task_id = None;

            debug!("Session state reset successfully");
            Ok(())
        }
    }
}
