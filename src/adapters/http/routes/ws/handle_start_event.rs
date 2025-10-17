use chrono::{DateTime, Utc};
use tracing::debug;
use uuid::Uuid;

use crate::{
    adapters::http::{
        app_state::AppState,
        dto::{common::session_type_enum::SessionTypeEnum, ws_msg::start_session_ws::StartSession},
        focus_sessions_state::FocusSessionState,
    },
    application::use_cases::commands::create_foucs_session::CreateFocusSessionCommand,
};

pub async fn handle_start_event(state: &AppState) -> Result<StartSession, String> {
    debug!("Handling start event");

    let mut session_state = state.focus_session_state.write().await;

    match session_state.current_session.clone() {
        Some(current_session) => {
            match current_session.session_type {
                SessionTypeEnum::Work => {
                    return Err(
                        "Work session already running cannot start a new session".to_string()
                    );
                }
                _ => {
                    let mut old_session_state = current_session.clone();
                    old_session_state.end_date = Some(Utc::now().timestamp());

                    session_state
                        .consecutive_sessions
                        .push(old_session_state.clone());

                    let task_id = old_session_state
                        .task_id
                        .as_ref()
                        .map(|id| {
                            Uuid::parse_str(id.as_str())
                                .map_err(|_| "Failed to parse task id".to_string())
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
                            actual_duration: old_session_state.end_date.unwrap()
                                - old_session_state.start_date, //should be safe since end date is setted in this function itself
                            started_at: DateTime::from_timestamp(old_session_state.start_date, 0)
                                .ok_or("Failed to parse session start time".to_string())?,
                            ended_at: DateTime::from_timestamp(
                                old_session_state.end_date.unwrap(),
                                0,
                            )
                            .ok_or("Failed to parse session end time".to_string())?,
                        })
                        .await;

                    let new_state = FocusSessionState {
                        session_type: SessionTypeEnum::Work,
                        start_date: Utc::now().timestamp(),
                        end_date: None,
                        category_id: session_state.workspace.category_id.clone(),
                        task_id: session_state.workspace.task_id.clone(),
                        note: None,
                        concentration_score: None,
                    };

                    session_state.current_session = Some(new_state.clone());

                    return Ok(StartSession {
                        session_type: new_state.session_type,
                        start_date: new_state.start_date,
                        category_id: new_state.category_id,
                        task_id: new_state.task_id,
                    });
                }
            }
        }
        None => {
            let new_state = FocusSessionState {
                session_type: SessionTypeEnum::Work,
                start_date: Utc::now().timestamp(),
                end_date: None,
                category_id: session_state.workspace.category_id.clone(),
                task_id: session_state.workspace.task_id.clone(),
                note: None,
                concentration_score: None,
            };

            session_state.current_session = Some(new_state.clone());
            return Ok(StartSession {
                session_type: SessionTypeEnum::Work,
                start_date: Utc::now().timestamp(),
                category_id: session_state.workspace.category_id.clone(),
                task_id: session_state.workspace.task_id.clone(),
            });
        }
    }
}
