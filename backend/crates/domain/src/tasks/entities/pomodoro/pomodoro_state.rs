use thiserror::Error;
use tracing::debug;
use uuid::Uuid;

use crate::tasks::entities::{
    focus_session::{
        FocusSession, FocusSessionError, NewSession, RunningSession, TerminatedSession,
    },
    focus_session_type::FocusSessionType,
};

const WORK_SESSIONS_BEFORE_LONG_BREAK: usize = 4;

#[derive(Debug, Error, PartialEq)]
pub enum PomodoroStateError {
    #[error("No running session")]
    NoRunningSession,

    #[error("Focus session error")]
    FocusSessionError(#[from] FocusSessionError),
}

pub type PomodoroStateResult<T> = Result<T, PomodoroStateError>;

#[derive(Debug, Default, Clone)]
pub struct PomodoroState {
    user_id: Uuid,
    task_id: Option<Uuid>,
    current_session: Option<FocusSession<RunningSession>>,
    consecutive_sessions: Vec<FocusSession<TerminatedSession>>,
}

impl PomodoroState {
    pub fn new() -> Self {
        Self {
            user_id: Uuid::default(),
            task_id: None,
            current_session: None,
            consecutive_sessions: Vec::new(),
        }
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn task_id(&self) -> Option<Uuid> {
        self.task_id
    }

    pub fn update_task_id(&mut self, task_id: Uuid) {
        self.task_id = Some(task_id);
    }

    pub fn current_session(&mut self) -> Option<FocusSession<RunningSession>> {
        self.current_session.clone()
    }

    pub fn update_current_session(&mut self, session: FocusSession<RunningSession>) {
        self.current_session = Some(session);
    }

    pub fn terminate_current_session(
        &mut self,
    ) -> PomodoroStateResult<FocusSession<TerminatedSession>> {
        if let Some(session) = self.current_session.take() {
            let terminated = session.terminate()?;
            self.consecutive_sessions.push(terminated.clone());
            self.current_session = None;
            self.task_id = None;
            Ok(terminated)
        } else {
            Err(PomodoroStateError::NoRunningSession)
        }
    }

    pub fn last_session(&self) -> Option<FocusSession<TerminatedSession>> {
        self.consecutive_sessions.last().cloned()
    }

    pub fn consecutive_sessions(&self) -> &[FocusSession<TerminatedSession>] {
        &self.consecutive_sessions
    }

    pub fn add_consecutive_session(&mut self, session: FocusSession<TerminatedSession>) {
        self.consecutive_sessions.push(session);
    }

    pub fn start_new_session(
        &mut self,
        user_id: Uuid,
        session_type: FocusSessionType,
        task_id: Option<Uuid>,
    ) -> PomodoroStateResult<()> {
        self.user_id = user_id;
        let new_session = FocusSession::<NewSession>::new(user_id, task_id, session_type)?;
        self.current_session = Some(new_session.run_session());
        Ok(())
    }

    /// Restores a previously running session from persisted state, preserving the original start time.
    pub fn restore_running_session(
        &mut self,
        user_id: Uuid,
        session: FocusSession<RunningSession>,
    ) {
        self.user_id = user_id;
        self.current_session = Some(session);
    }

    /// Calculates the next session type based on Pomodoro technique rules:
    /// - After completing a Work session: ShortBreak or LongBreak
    /// - After completing a Break session: Work
    /// - LongBreak is suggested after 4 completed Work sessions
    pub fn calculate_next_session_type(&self) -> FocusSessionType {
        let last_type = self
            .consecutive_sessions
            .last()
            .map(|s| s.session_type())
            .or_else(|| self.current_session.as_ref().map(|s| s.session_type()));

        let last_session_type = match last_type {
            Some(t) => t,
            None => {
                debug!("No session history or current session, defaulting to Work");
                return FocusSessionType::Work;
            }
        };

        debug!("Last session type: {:?}", last_session_type);

        match last_session_type {
            FocusSessionType::Work => {
                let mut completed_work_sessions = self
                    .consecutive_sessions
                    .iter()
                    .filter(|s| s.session_type() == FocusSessionType::Work)
                    .count();

                // Include the current session if it exists
                if self.current_session.is_some() {
                    completed_work_sessions += 1;
                }
                debug!(
                    "Completed work sessions: {}, work sessions before long break: {}",
                    completed_work_sessions, WORK_SESSIONS_BEFORE_LONG_BREAK
                );
                let next_type = if completed_work_sessions % WORK_SESSIONS_BEFORE_LONG_BREAK == 0 {
                    FocusSessionType::LongBreak
                } else {
                    FocusSessionType::ShortBreak
                };

                debug!(
                    "After work session #{}, suggesting: {:?}",
                    completed_work_sessions, next_type
                );

                next_type
            }
            FocusSessionType::ShortBreak | FocusSessionType::LongBreak => {
                debug!("After break, suggesting work session");
                FocusSessionType::Work
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user_id() -> Uuid {
        Uuid::new_v4()
    }

    fn state_with_n_work_sessions(n: usize) -> PomodoroState {
        let mut state = PomodoroState::default();
        for _ in 0..n {
            state
                .start_new_session(user_id(), FocusSessionType::Work, None)
                .unwrap();
            std::thread::sleep(std::time::Duration::from_secs(1));
            state.terminate_current_session().unwrap();
        }
        state
    }

    #[test]
    fn test_start_new_session_sets_current_session() {
        let mut state = PomodoroState::default();
        let result = state.start_new_session(user_id(), FocusSessionType::Work, None);
        assert!(result.is_ok());
        assert!(state.current_session().is_some());
    }

    #[test]
    fn test_start_new_session_sets_user_id() {
        let mut state = PomodoroState::default();
        let uid = user_id();
        state
            .start_new_session(uid, FocusSessionType::Work, None)
            .unwrap();
        assert_eq!(state.user_id(), uid);
    }

    #[test]
    fn test_terminate_current_session_ok() {
        let mut state = PomodoroState::default();
        state
            .start_new_session(user_id(), FocusSessionType::Work, None)
            .unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert!(state.terminate_current_session().is_ok());
    }

    #[test]
    fn test_terminate_current_session_clears_current() {
        let mut state = PomodoroState::default();
        state
            .start_new_session(user_id(), FocusSessionType::Work, None)
            .unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
        state.terminate_current_session().unwrap();
        assert!(state.current_session().is_none());
    }

    #[test]
    fn test_terminate_current_session_adds_to_history() {
        let mut state = PomodoroState::default();
        state
            .start_new_session(user_id(), FocusSessionType::Work, None)
            .unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
        state.terminate_current_session().unwrap();
        assert!(state.last_session().is_some());
    }

    #[test]
    fn test_terminate_without_running_session_fails() {
        let mut state = PomodoroState::default();
        assert!(matches!(
            state.terminate_current_session(),
            Err(PomodoroStateError::NoRunningSession)
        ));
    }

    #[test]
    fn test_last_session_none_on_empty() {
        let state = PomodoroState::default();
        assert!(state.last_session().is_none());
    }

    #[test]
    fn test_last_session_returns_most_recent() {
        let mut state = PomodoroState::default();
        state
            .start_new_session(user_id(), FocusSessionType::Work, None)
            .unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
        state.terminate_current_session().unwrap();
        state
            .start_new_session(user_id(), FocusSessionType::ShortBreak, None)
            .unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
        state.terminate_current_session().unwrap();
        assert_eq!(
            state.last_session().unwrap().session_type(),
            FocusSessionType::ShortBreak
        );
    }

    #[test]
    fn test_update_task_id() {
        let mut state = PomodoroState::default();
        let id = Uuid::new_v4();
        state.update_task_id(id);
        assert_eq!(state.task_id(), Some(id));
    }

    #[test]
    fn test_next_session_type_empty_returns_work() {
        let state = PomodoroState::default();
        assert_eq!(state.calculate_next_session_type(), FocusSessionType::Work);
    }

    #[test]
    fn test_next_session_type_after_1_work_is_short_break() {
        let state = state_with_n_work_sessions(1);
        assert_eq!(
            state.calculate_next_session_type(),
            FocusSessionType::ShortBreak
        );
    }

    #[test]
    fn test_next_session_type_after_4_work_is_long_break() {
        let state = state_with_n_work_sessions(4);
        assert_eq!(
            state.calculate_next_session_type(),
            FocusSessionType::LongBreak
        );
    }

    #[test]
    fn test_next_session_type_after_8_work_is_long_break() {
        let state = state_with_n_work_sessions(8);
        assert_eq!(
            state.calculate_next_session_type(),
            FocusSessionType::LongBreak
        );
    }

    #[test]
    fn test_next_session_type_after_short_break_is_work() {
        let mut state = state_with_n_work_sessions(1);
        state
            .start_new_session(user_id(), FocusSessionType::ShortBreak, None)
            .unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
        state.terminate_current_session().unwrap();
        assert_eq!(state.calculate_next_session_type(), FocusSessionType::Work);
    }

    #[test]
    fn test_next_session_type_after_long_break_is_work() {
        let mut state = state_with_n_work_sessions(4);
        state
            .start_new_session(user_id(), FocusSessionType::LongBreak, None)
            .unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
        state.terminate_current_session().unwrap();
        assert_eq!(state.calculate_next_session_type(), FocusSessionType::Work);
    }
}
