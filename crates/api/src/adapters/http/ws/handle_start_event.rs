use chrono::{DateTime, Utc};
use tracing::debug;

use crate::adapters::http::{
    app_state::AppState, dto::common::session_type_enum::SessionTypeEnum,
    ws::update_pomodoro_state::UpdatePomodoroState,
};
use application::use_cases::focus_session::command::create_foucs_session::CreateFocusSessionCommand;

pub async fn handle_start_event(state: &AppState) -> Result<UpdatePomodoroState, String> {
    debug!("Handling start session event");

    let mut session_state = state.pomodoro_state.write().await;

    match session_state.current_session() {
        Some(current_session) => match current_session.session_type() {
            SessionTypeEnum::Work => {
                Err("Work session already running cannot start a new session".to_string())
            }
            _ => {
                session_state.close_current_session(Utc::now().timestamp())?;

                if let Some(last_session) = session_state.last_session() {
                    let _ = state
                        .create_session_usecase
                        .execute(CreateFocusSessionCommand {
                            // Break session should not have a task or category
                            task_id: None,
                            category_id: None,
                            session_type: last_session.session_type().clone().into(),
                            concentration_score: last_session.concentration_score(),
                            notes: last_session.note().clone(),
                            actual_duration: last_session.end_date().unwrap()
                                - last_session.start_date(), //should be safe since end date is setted in this function itself
                            started_at: DateTime::from_timestamp(last_session.start_date(), 0)
                                .ok_or("Failed to parse session start time".to_string())?,
                            ended_at: DateTime::from_timestamp(last_session.end_date().unwrap(), 0)
                                .ok_or("Failed to parse session end time".to_string())?,
                        })
                        .await;
                }

                let category_id = session_state.current_work_context().category_id().cloned();
                let task_id = session_state.current_work_context().task_id().cloned();

                let _ = session_state.start_new_session(
                    SessionTypeEnum::Work,
                    Utc::now().timestamp(),
                    category_id,
                    task_id,
                );

                Ok(UpdatePomodoroState::from(session_state.clone()))
            }
        },
        None => {
            let category_id = session_state.current_work_context().category_id().cloned();
            let task_id = session_state.current_work_context().task_id().cloned();

            let _ = session_state.start_new_session(
                SessionTypeEnum::Work,
                Utc::now().timestamp(),
                category_id,
                task_id,
            );
            Ok(UpdatePomodoroState::from(session_state.clone()))
        }
    }
}
