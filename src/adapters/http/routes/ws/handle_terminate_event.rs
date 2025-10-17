use chrono::{DateTime, Utc};
use tracing::debug;
use uuid::Uuid;

use crate::{
    adapters::http::app_state::AppState,
    application::use_cases::commands::create_foucs_session::CreateFocusSessionCommand,
};

pub async fn handle_terminate_event(state: &AppState) -> Result<(), String> {
    debug!("Handling terminate event");

    let mut session_state = state.focus_session_state.write().await;

    match session_state.current_session.clone() {
        Some(current_session) => {
            let mut old_session_state = current_session.clone();
            old_session_state.end_date = Some(Utc::now().timestamp());

            let task_id = old_session_state
                .task_id
                .as_ref()
                .map(|id| {
                    Uuid::parse_str(id.as_str()).map_err(|_| "Failed to parse task id".to_string())
                })
                .transpose()?;

            let category_id = old_session_state
                .category_id
                .as_ref()
                .map(|id| {
                    Uuid::parse_str(id.as_str())
                        .map_err(|_| "Failed to parse category id".to_string())
                })
                .transpose()?;

            let _ = state
                .focus_session_use_cases
                .create_session(CreateFocusSessionCommand {
                    task_id,
                    category_id,
                    session_type: old_session_state.session_type.into(),
                    concentration_score: old_session_state.concentration_score,
                    notes: old_session_state.note.clone(),
                    actual_duration: old_session_state.end_date.unwrap() // should be safe since it is setted in this scope
                                                    - old_session_state.start_date,
                    started_at: DateTime::from_timestamp(old_session_state.start_date, 0)
                        .ok_or("Failed to parse session start time".to_string())?,
                    ended_at: DateTime::from_timestamp(old_session_state.end_date.unwrap(), 0)
                        .ok_or("Failed to parse session end time".to_string())?,
                })
                .await;

            session_state.current_session = None;
            session_state.consecutive_sessions = Vec::new();

            Ok(())
        }
        None => {
            return Err("No active sessions, cannot terminate a session".to_string());
        }
    }
}
