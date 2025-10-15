use chrono::{DateTime, Utc};
use tracing::debug;
use uuid::Uuid;

use crate::{
    adapters::http::{
        app_state::AppState,
        dto::{
            common::session_type_enum::SessionTypeEnum,
            ws_msg::{complete_session_ws::CompleteSession, start_session_ws::StartSession},
        },
        focus_sessions_state::FocusSessionState,
    },
    application::use_cases::commands::create_foucs_session::CreateFocusSessionCommand,
};

const WORK_SESSIONS_BEFORE_LONG_BREAK: usize = 4;

/// Completes the current session, persists it to the database,
/// and calculates the next session based on Pomodoro rules.
/// Returns the suggested next session to broadcast to clients.
pub async fn complete_session(
    message: &CompleteSession,
    state: &AppState,
) -> Result<Option<StartSession>, String> {
    debug!("Completing session");

    let mut session_state = state.focus_session_state.write().await;

    match &session_state.current_session {
        Some(running_session) => {
            let mut old_session_state = running_session.clone();
            old_session_state.end_date = Some(Utc::now().timestamp());

            session_state
                .consecutive_sessions
                .push(old_session_state.clone());

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
                    concentration_score: message.concentration_score,
                    notes: old_session_state.note.clone(),
                    actual_duration: message.actual_duration,
                    started_at: DateTime::from_timestamp(old_session_state.start_date, 0)
                        .ok_or("Failed to parse session start time".to_string())?,
                    ended_at: DateTime::from_timestamp(old_session_state.end_date.unwrap(), 0)
                        .ok_or("Failed to parse session end time".to_string())?,
                })
                .await;

            session_state.current_session = None;

            let next_session = calculate_next_session(
                &session_state.consecutive_sessions,
                &old_session_state.category_id,
                &old_session_state.task_id,
            );

            debug!("Next session calculated: {:?}", next_session);

            Ok(next_session)
        }
        None => Err("No running session to complete".to_string()),
    }
}

/// Calculates the next session type based on Pomodoro technique rules:
/// - After completing a Work session: ShortBreak or LongBreak
/// - After completing a Break session: Work
/// - LongBreak is suggested after 4 completed Work sessions
fn calculate_next_session(
    consecutive_sessions: &[FocusSessionState],
    category_id: &Option<String>,
    task_id: &Option<String>,
) -> Option<StartSession> {
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

            Some(StartSession {
                session_type: next_type,
                start_date: Utc::now().timestamp(),
                category_id: category_id.clone(),
                task_id: task_id.clone(),
            })
        }
        SessionTypeEnum::ShortBreak | SessionTypeEnum::LongBreak => {
            debug!("After break, suggesting work session");

            Some(StartSession {
                session_type: SessionTypeEnum::Work,
                start_date: Utc::now().timestamp(),
                category_id: category_id.clone(),
                task_id: task_id.clone(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::adapters::http::focus_sessions_state::FocusSessionState;

    use super::*;

    fn create_test_session(session_type: SessionTypeEnum) -> FocusSessionState {
        FocusSessionState {
            session_type,
            start_date: Utc::now().timestamp(),
            end_date: Some(Utc::now().timestamp() + 1500),
            category_id: Some("test-category".to_string()),
            task_id: Some("test-task".to_string()),
            note: None,
        }
    }

    #[test]
    fn test_first_work_session_suggests_short_break() {
        let sessions = vec![create_test_session(SessionTypeEnum::Work)];
        let next = calculate_next_session(
            &sessions,
            &Some("test-category".to_string()),
            &Some("test-task".to_string()),
        );

        assert!(next.is_some());
        let next = next.unwrap();
        assert_eq!(next.session_type, SessionTypeEnum::ShortBreak);
    }

    #[test]
    fn test_fourth_work_session_suggests_long_break() {
        let sessions = vec![
            create_test_session(SessionTypeEnum::Work),
            create_test_session(SessionTypeEnum::ShortBreak),
            create_test_session(SessionTypeEnum::Work),
            create_test_session(SessionTypeEnum::ShortBreak),
            create_test_session(SessionTypeEnum::Work),
            create_test_session(SessionTypeEnum::ShortBreak),
            create_test_session(SessionTypeEnum::Work),
        ];

        let next = calculate_next_session(
            &sessions,
            &Some("test-category".to_string()),
            &Some("test-task".to_string()),
        );

        assert!(next.is_some());
        let next = next.unwrap();
        assert_eq!(next.session_type, SessionTypeEnum::LongBreak);
    }

    #[test]
    fn test_after_break_suggests_work() {
        let sessions = vec![
            create_test_session(SessionTypeEnum::Work),
            create_test_session(SessionTypeEnum::ShortBreak),
        ];

        let next = calculate_next_session(
            &sessions,
            &Some("test-category".to_string()),
            &Some("test-task".to_string()),
        );

        assert!(next.is_some());
        let next = next.unwrap();
        assert_eq!(next.session_type, SessionTypeEnum::Work);
    }

    #[test]
    fn test_after_long_break_suggests_work() {
        let sessions = vec![
            create_test_session(SessionTypeEnum::Work),
            create_test_session(SessionTypeEnum::LongBreak),
        ];

        let next = calculate_next_session(
            &sessions,
            &Some("test-category".to_string()),
            &Some("test-task".to_string()),
        );

        assert!(next.is_some());
        let next = next.unwrap();
        assert_eq!(next.session_type, SessionTypeEnum::Work);
    }

    #[test]
    fn test_preserves_category_and_task() {
        let sessions = vec![create_test_session(SessionTypeEnum::Work)];
        let next = calculate_next_session(
            &sessions,
            &Some("my-category".to_string()),
            &Some("my-task".to_string()),
        );

        assert!(next.is_some());
        let next = next.unwrap();
        assert_eq!(next.category_id, Some("my-category".to_string()));
        assert_eq!(next.task_id, Some("my-task".to_string()));
    }
}
