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

    if session_state.current_session.is_some() {
        return Err("A session is already running. Please complete it first.".to_string());
    }

    let focus_session_state = FocusSessionState {
        session_type: message.session_type.clone(),
        start_date: message.start_date,
        category_id: message.category_id.clone(),
        task_id: message.task_id.clone(),
        note: None,
        end_date: None,
    };

    session_state.current_session = Some(focus_session_state);

    Ok(message.clone())
}
