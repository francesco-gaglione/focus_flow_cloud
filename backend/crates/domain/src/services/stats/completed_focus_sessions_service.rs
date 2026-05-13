use chrono::Duration;

use crate::{
    entities::focus_session::{FocusSession, TerminatedSession},
    value_objects::stats::completed_focus_sessions::CompletedFocusSessions,
};

pub struct CompletedFocusSessionsService {}

impl CompletedFocusSessionsService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn calculate(sessions: &[FocusSession<TerminatedSession>]) -> CompletedFocusSessions {
        let count = sessions.len();
        let avg_duration = if count == 0 {
            Duration::zero()
        } else {
            let total_secs: i64 = sessions.iter().map(|s| s.actual_duration()).sum();
            Duration::seconds(total_secs / count as i64)
        };
        CompletedFocusSessions::new(count, avg_duration)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};
    use uuid::Uuid;

    use crate::entities::{
        focus_session::{FocusSession, TerminatedSession},
        focus_session_type::FocusSessionType,
    };

    use super::*;

    fn session(duration_secs: i64) -> FocusSession<TerminatedSession> {
        let started_at = Utc::now() - Duration::seconds(duration_secs);
        let ended_at = Utc::now();
        FocusSession::<TerminatedSession>::new(
            Uuid::new_v4(),
            None,
            FocusSessionType::Work,
            None,
            None,
            started_at,
            ended_at,
        )
        .unwrap()
    }

    #[test]
    fn test_empty() {
        let result = CompletedFocusSessionsService::calculate(&[]);
        assert_eq!(result.count(), 0);
        assert_eq!(result.avg_duration(), Duration::zero());
    }

    #[test]
    fn test_single_session() {
        let sessions = vec![session(3600)];
        let result = CompletedFocusSessionsService::calculate(&sessions);
        assert_eq!(result.count(), 1);
        assert_eq!(result.avg_duration().num_seconds(), 3600);
    }

    #[test]
    fn test_avg_duration() {
        let sessions = vec![session(1800), session(3600)];
        let result = CompletedFocusSessionsService::calculate(&sessions);
        assert_eq!(result.count(), 2);
        assert_eq!(result.avg_duration().num_seconds(), 2700);
    }
}
