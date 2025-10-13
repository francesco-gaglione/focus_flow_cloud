use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::debug;

use crate::adapters::http::{
    dto::ws_msg::start_session_ws::StartSession,
    focus_sessions_state::{FocusSessionState, FocusSessionsState},
};

pub async fn start_session(
    message: &StartSession,
    sessions_state: &Arc<RwLock<FocusSessionsState>>,
) -> Result<StartSession, String> {
    debug!("Starting session");

    let mut state = sessions_state.write().await;

    match state.current_session {
        Some(_) => Err("Session already running".to_string()),
        None => {
            state.current_session = Some(FocusSessionState {
                session_type: message.session_type.clone(),
                start_date: message.start_date,
                end_date: None,
                category_id: message.category_id.clone(),
                task_id: message.task_id.clone(),
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
