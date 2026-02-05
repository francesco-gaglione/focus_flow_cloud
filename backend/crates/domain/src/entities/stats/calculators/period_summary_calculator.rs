use crate::entities::focus_session::FocusSession;
use crate::entities::focus_session_type::FocusSessionType;

#[derive(Debug, Clone)]
pub struct PeriodSummary {
    pub total_sessions: usize,
    pub total_breaks: usize,
    pub total_focus_time: i64,
    pub total_break_time: i64,
}

pub struct PeriodSummaryCalculator;

impl PeriodSummaryCalculator {
    pub fn calculate(sessions: &[FocusSession]) -> PeriodSummary {
        let (work_sessions, break_sessions): (Vec<_>, Vec<_>) = sessions
            .iter()
            .partition(|s| s.session_type() == FocusSessionType::Work);

        let total_sessions = work_sessions.len();
        let total_breaks = break_sessions.len();

        let total_focus_time: i64 = work_sessions
            .iter()
            .filter_map(|s| s.actual_duration())
            .sum();

        let total_break_time: i64 = break_sessions
            .iter()
            .filter_map(|s| s.actual_duration())
            .sum();

        PeriodSummary {
            total_sessions,
            total_breaks,
            total_focus_time,
            total_break_time,
        }
    }
}
