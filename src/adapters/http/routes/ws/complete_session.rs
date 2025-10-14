use chrono::{DateTime, Utc};
use tracing::debug;
use uuid::Uuid;

use crate::{
    adapters::http::{app_state::AppState, dto::ws_msg::complete_session_ws::CompleteSession},
    application::use_cases::commands::create_foucs_session::CreateFocusSessionCommand,
};

pub async fn complete_session(message: &CompleteSession, state: &AppState) -> Result<(), String> {
    debug!("Starting session");

    let mut session_state = state.focus_session_state.write().await;

    match &session_state.current_session {
        Some(running_session) => {
            //TODO check consecutive session and calulate long or short break

            let mut old_session_state = running_session.clone();
            old_session_state.end_date = Some(Utc::now().timestamp());
            session_state
                .consecutive_sessions
                .push(old_session_state.clone());

            let task_id = old_session_state
                .task_id
                .map(|id| {
                    Uuid::parse_str(id.as_str()).map_err(|e| "Failed to parse task id".to_string())
                })
                .transpose()?;

            let category_id = old_session_state
                .category_id
                .map(|id| {
                    Uuid::parse_str(id.as_str()).map_err(|e| "Failed to parse task id".to_string())
                })
                .transpose()?;

            state
                .focus_session_use_cases
                .create_session(CreateFocusSessionCommand {
                    task_id,
                    category_id,
                    session_type: old_session_state.session_type.into(),
                    concentration_score: message.concentration_score,
                    notes: old_session_state.note,
                    actual_duration: message.actual_duration,
                    started_at: DateTime::from_timestamp(old_session_state.start_date, 0)
                        .ok_or("Failed to parse session start time".to_string())?,
                    ended_at: DateTime::from_timestamp(old_session_state.end_date.unwrap(), 0)
                        .ok_or("Failed to parse session end time".to_string())?,
                })
                .await;

            session_state.current_session = None;

            Ok(())
        }
        None => Err("No running session to complete".to_string()),
    }
}
