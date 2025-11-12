use chrono::{DateTime, Utc};
use tracing::debug;
use uuid::Uuid;

use crate::{
    adapters::http::{
        app_state::AppState,
        dto::{common::session_type_enum::SessionTypeEnum, ws_msg::start_session_ws::StartSession},
        pomodoro_state::FocusSessionState,
    },
    application::use_cases::commands::create_foucs_session::CreateFocusSessionCommand,
};

pub async fn handle_break_event(state: &AppState) -> Result<StartSession, String> {
    debug!("Handling pause event");

    let mut pomodoro_state = state.pomodoro_state.write().await;

    match pomodoro_state.current_session_type() {
        Some(current_session_type) => match current_session_type {
            SessionTypeEnum::Work => {
                pomodoro_state.close_current_session(Utc::now().timestamp());

                match pomodoro_state.last_session() {
                    Some(last_session) => {
                        let task_id = last_session
                            .task_id()
                            .map(|id| {
                                Uuid::parse_str(id.as_str())
                                    .map_err(|_| "Failed to parse task id".to_string())
                            })
                            .transpose()?;

                        let category_id = last_session
                            .category_id()
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
                                session_type: last_session.session_type().clone().into(),
                                concentration_score: last_session.concentration_score(),
                                notes: last_session.note().clone(),
                                actual_duration: last_session.actual_duration().unwrap(), // should be safe but improve it
                                started_at: DateTime::from_timestamp(last_session.start_date(), 0)
                                    .ok_or("Failed to parse session start time".to_string())?,
                                ended_at: DateTime::from_timestamp(
                                    last_session.end_date().unwrap(), // Should be safe but improve it
                                    0,
                                )
                                .ok_or("Failed to parse session end time".to_string())?,
                            })
                            .await;
                    }
                    None => todo!(),
                }

                let next_session_type = pomodoro_state.calculate_next_session_type();

                pomodoro_state.start_new_session(
                    next_session_type,
                    Utc::now().timestamp(),
                    category_id,
                    task_id,
                );

                Ok(StartSession {
                    session_type: new_state.session_type,
                    start_date: new_state.start_date,
                    category_id: new_state.category_id,
                    task_id: new_state.task_id,
                })
            }
            _ => {
                tracing::error!("Break session already running cannot start a new break");
                Err("Break session already running cannot start a new break".to_string())
            }
        },
        None => {
            tracing::error!("No active sessions, cannot calculate a break");
            Err("No active sessions, cannot calculate a break".to_string())
        }
    }
}
