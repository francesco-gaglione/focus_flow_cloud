use chrono::{DateTime, Utc};
use tracing::debug;
use uuid::Uuid;

use crate::{
    adapters::http::{
        app_state::AppState, dto::ws_msg::update_pomodoro_state::UpdatePomodoroState,
    },
    application::use_cases::commands::create_foucs_session::CreateFocusSessionCommand,
};

pub async fn handle_terminate_event(state: &AppState) -> Result<UpdatePomodoroState, String> {
    debug!("Handling terminate event");

    let mut session_state = state.pomodoro_state.write().await;

    session_state.close_current_session(Utc::now().timestamp())?;

    if let Some(old_session_state) = session_state.last_session() {
        let task_id = old_session_state
            .task_id()
            .map(|id| {
                Uuid::parse_str(id.as_str()).map_err(|_| "Failed to parse task id".to_string())
            })
            .transpose()?;

        let category_id = old_session_state
            .category_id()
            .map(|id| {
                Uuid::parse_str(id.as_str()).map_err(|_| "Failed to parse category id".to_string())
            })
            .transpose()?;

        let _ = state
            .focus_session_use_cases
            .create_session(CreateFocusSessionCommand {
                task_id,
                category_id,
                session_type: old_session_state.session_type().clone().into(),
                concentration_score: old_session_state.concentration_score(),
                notes: old_session_state.note(),
                actual_duration: old_session_state
                    .actual_duration()
                    .ok_or("Failed to calculate session duration".to_string())?,
                started_at: DateTime::from_timestamp(old_session_state.start_date(), 0)
                    .ok_or("Failed to parse session start time".to_string())?,
                ended_at: DateTime::from_timestamp(
                    old_session_state
                        .end_date()
                        .ok_or("Session end date not set".to_string())?,
                    0,
                )
                .ok_or("Failed to parse session end time".to_string())?,
            })
            .await
            .map_err(|e| {
                debug!("Error creating a new session: {:?}", e);
                "Error creating a new session".to_string()
            })?;
    }

    Ok(UpdatePomodoroState::from(session_state.clone()))
}
