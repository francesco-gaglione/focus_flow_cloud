use tracing::debug;

use crate::adapters::http::{
    app_state::AppState, dto::ws_msg::start_session_ws::StartSession,
    focus_sessions_state::FocusSessionState,
};

pub async fn start_session(
    message: &StartSession,
    state: &AppState,
) -> Result<StartSession, String> {
    debug!("Starting session");

    let mut session_state = state.focus_session_state.write().await;

    match &session_state.current_session {
        Some(_) => {
            return Err("Session already running".to_string());
        }
        None => {
            session_state.current_session = Some(FocusSessionState {
                session_type: message.session_type.clone(),
                start_date: message.start_date,
                end_date: None,
                category_id: message.category_id.clone(),
                task_id: message.task_id.clone(),
                note: None,
            });
            Ok(StartSession {
                session_type: message.session_type.clone(),
                start_date: message.start_date,
                category_id: message.category_id.clone(),
                task_id: message.task_id.clone(),
            })
        }
    }
}
