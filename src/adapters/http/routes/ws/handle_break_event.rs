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

pub async fn handle_break_event(state: &AppState) -> Result<StartSession, String> {
    debug!("Handling pause event");

    let mut session_state = state.focus_session_state.write().await;

    match session_state.current_session.clone() {
        Some(current_session) => match current_session.session_type {
            SessionTypeEnum::Work => {
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
                        actual_duration: old_session_state.end_date.unwrap() // should be safe since it is setted in this scope
                            - old_session_state.start_date,
                        started_at: DateTime::from_timestamp(old_session_state.start_date, 0)
                            .ok_or("Failed to parse session start time".to_string())?,
                        ended_at: DateTime::from_timestamp(old_session_state.end_date.unwrap(), 0)
                            .ok_or("Failed to parse session end time".to_string())?,
                    })
                    .await;

                let next_session_type =
                    calculate_next_session_type(&session_state.consecutive_sessions);

                let new_state = FocusSessionState {
                    session_type: next_session_type.unwrap_or(SessionTypeEnum::ShortBreak),
                    start_date: Utc::now().timestamp(),
                    end_date: None,
                    category_id: session_state.workspace.category_id.clone(),
                    task_id: session_state.workspace.task_id.clone(),
                    note: None,
                    concentration_score: None,
                };

                session_state.current_session = Some(new_state.clone());

                Ok(StartSession {
                    session_type: new_state.session_type,
                    start_date: new_state.start_date,
                    category_id: new_state.category_id,
                    task_id: new_state.task_id,
                })
            }
            _ => {
                return Err("Break session already running cannot start a new break".to_string());
            }
        },
        None => {
            return Err("No active sessions, cannot calculate a break".to_string());
        }
    }
}

const WORK_SESSIONS_BEFORE_LONG_BREAK: usize = 4;

/// Calculates the next session type based on Pomodoro technique rules:
/// - After completing a Work session: ShortBreak or LongBreak
/// - After completing a Break session: Work
/// - LongBreak is suggested after 4 completed Work sessions
fn calculate_next_session_type(
    consecutive_sessions: &[FocusSessionState],
) -> Option<SessionTypeEnum> {
    if consecutive_sessions.is_empty() {
        return None;
    }

    let last_session = consecutive_sessions.last()?;

    match last_session.session_type {
        SessionTypeEnum::Work => {
            let completed_work_sessions = consecutive_sessions
                .iter()
                .filter(|s| s.session_type == SessionTypeEnum::Work)
                .count();

            let next_type = if completed_work_sessions % WORK_SESSIONS_BEFORE_LONG_BREAK == 0 {
                SessionTypeEnum::LongBreak
            } else {
                SessionTypeEnum::ShortBreak
            };

            debug!(
                "After work session #{}, suggesting: {:?}",
                completed_work_sessions, next_type
            );

            Some(next_type)
        }
        SessionTypeEnum::ShortBreak | SessionTypeEnum::LongBreak => {
            debug!("After break, suggesting work session");
            Some(SessionTypeEnum::Work)
        }
    }
}
