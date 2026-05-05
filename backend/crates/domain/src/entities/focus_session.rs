use std::marker::PhantomData;

use crate::entities::focus_session_type::FocusSessionType;
use chrono::{DateTime, Utc};
use thiserror::Error;
use tracing::debug;
use uuid::Uuid;

#[derive(Debug, Error, PartialEq)]
pub enum FocusSessionError {
    #[error("Invalid concentration score: {0}")]
    InvalidConcentrationScore(i32),

    #[error("Invalid start date")]
    StartDateMissing,

    #[error("Invalid date range: {0}")]
    InvalidDateRange(String),
}

pub type FocusSessionResult<T> = Result<T, FocusSessionError>;

/// Marker type: rapresent a running session (start at has a value, ended at not)
#[derive(Debug, Clone)]
pub struct RunningSession;

/// Marker type: a terminated session (endnd at has a value)
#[derive(Debug, Clone)]
pub struct TerminatedSession;

/// Marker type: new created session, ready to be executed
#[derive(Debug, Clone)]
pub struct NewSession;

#[derive(Debug, Clone)]
pub struct FocusSession<S> {
    id: Uuid,
    user_id: Uuid,
    task_id: Option<Uuid>,
    session_type: FocusSessionType,
    actual_duration: Option<i64>,
    concentration_score: Option<i32>,
    notes: Option<String>,
    started_at: Option<DateTime<Utc>>,
    ended_at: Option<DateTime<Utc>>,
    _state: PhantomData<S>,
}

impl<S> FocusSession<S> {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn session_type(&self) -> FocusSessionType {
        self.session_type
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn task_id(&self) -> Option<Uuid> {
        self.task_id
    }

    pub fn update_task_id(&mut self, id: Uuid) {
        self.task_id = Some(id);
    }

    pub fn note(&self) -> Option<String> {
        self.notes.clone()
    }

    pub fn update_note(&mut self, note: String) {
        self.notes = Some(note)
    }
}

impl FocusSession<NewSession> {
    pub fn new(
        user_id: Uuid,
        task_id: Option<Uuid>,
        session_type: FocusSessionType,
    ) -> FocusSessionResult<Self> {
        Ok(FocusSession {
            id: Uuid::new_v4(),
            user_id,
            task_id,
            session_type,
            actual_duration: None,
            concentration_score: None,
            notes: None,
            started_at: None,
            ended_at: None,
            _state: PhantomData,
        })
    }

    /// Transitions the session to the `RunningSession` state and sets the start time.
    ///
    /// This consumes the current session and returns a new one with the updated state.
    pub fn run_session(&self) -> FocusSession<RunningSession> {
        let started_at = Utc::now();
        FocusSession {
            id: self.id,
            user_id: self.user_id,
            task_id: self.task_id,
            session_type: self.session_type,
            actual_duration: self.actual_duration,
            concentration_score: self.concentration_score,
            notes: self.notes.clone(),
            started_at: Some(started_at),
            ended_at: self.ended_at,
            _state: PhantomData,
        }
    }
}

impl FocusSession<RunningSession> {
    pub fn update_concentration_score(
        &mut self,
        concentration_score: i32,
    ) -> FocusSessionResult<()> {
        if !(0..=5).contains(&concentration_score) {
            return Err(FocusSessionError::InvalidConcentrationScore(
                concentration_score,
            ));
        }
        self.concentration_score = Some(concentration_score);
        Ok(())
    }

    /// Terminates the session and calculates the actual duration.
    ///
    /// This consumes the `RunningSession` and returns a `TerminatedSession`.
    ///
    /// # Errors
    ///
    /// Returns [`FocusSessionError::StartDateMissing`] if the start time was not set,
    /// or [`FocusSessionError::InvalidDateRange`] if the current time is before the start time.
    pub fn terminate(&self) -> FocusSessionResult<FocusSession<TerminatedSession>> {
        let now = Utc::now();
        let started_at = match self.started_at {
            Some(s) => s.timestamp(),
            None => return Err(FocusSessionError::StartDateMissing),
        };

        if now.timestamp() <= started_at {
            debug!(
                "Start date is in the future: now={:?}, started_at={:?}",
                now.timestamp(),
                started_at
            );
            return Err(FocusSessionError::InvalidDateRange(
                "Start date is in the future".to_string(),
            ));
        }

        let actual_duration = now.timestamp() - started_at;

        Ok(FocusSession {
            id: self.id,
            user_id: self.user_id,
            task_id: self.task_id,
            session_type: self.session_type,
            actual_duration: Some(actual_duration),
            concentration_score: self.concentration_score,
            notes: self.notes.clone(),
            started_at: self.started_at,
            ended_at: Some(now),
            _state: PhantomData,
        })
    }

    pub fn started_at(&self) -> DateTime<Utc> {
        self.started_at.unwrap()
    }
}

impl FocusSession<TerminatedSession> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        user_id: Uuid,
        task_id: Option<Uuid>,
        session_type: FocusSessionType,
        concentration_score: Option<i32>,
        note: Option<String>,
        started_at: DateTime<Utc>,
        ended_at: DateTime<Utc>,
    ) -> FocusSessionResult<Self> {
        if started_at > ended_at {
            return Err(FocusSessionError::InvalidDateRange(
                "Invalid date range".to_string(),
            ));
        }

        let duration = (ended_at - started_at).as_seconds_f32() as i64;

        Ok(FocusSession {
            id: Uuid::new_v4(),
            user_id,
            task_id,
            session_type,
            actual_duration: Some(duration),
            concentration_score,
            notes: note,
            started_at: Some(started_at),
            ended_at: Some(ended_at),
            _state: PhantomData,
        })
    }

    pub fn actual_duration(&self) -> i64 {
        self.actual_duration.unwrap()
    }

    pub fn concentration_score(&self) -> Option<i32> {
        self.concentration_score
    }

    pub fn update_concentration_score(
        &mut self,
        concentration_score: i32,
    ) -> FocusSessionResult<()> {
        if !(0..=5).contains(&concentration_score) {
            return Err(FocusSessionError::InvalidConcentrationScore(
                concentration_score,
            ));
        }
        self.concentration_score = Some(concentration_score);
        Ok(())
    }

    pub fn started_at(&self) -> DateTime<Utc> {
        self.started_at.unwrap()
    }

    pub fn ended_at(&self) -> DateTime<Utc> {
        self.ended_at.unwrap()
    }

    /// Reconstitutes a `TerminatedSession` from persisted data, preserving the original ID.
    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: Uuid,
        user_id: Uuid,
        task_id: Option<Uuid>,
        session_type: FocusSessionType,
        actual_duration: Option<i64>,
        concentration_score: Option<i32>,
        notes: Option<String>,
        started_at: DateTime<Utc>,
        ended_at: Option<DateTime<Utc>>,
    ) -> Self {
        FocusSession {
            id,
            user_id,
            task_id,
            session_type,
            actual_duration,
            concentration_score,
            notes,
            started_at: Some(started_at),
            ended_at,
            _state: PhantomData,
        }
    }

    pub fn update_date_range(
        &mut self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> FocusSessionResult<()> {
        if start_date > end_date {
            return Err(FocusSessionError::InvalidDateRange(format!(
                "Invalid date range. Start date: {}, end date: {}",
                start_date, end_date
            )));
        }

        self.started_at = Some(start_date);
        self.ended_at = Some(end_date);
        let duration = end_date - start_date;
        self.actual_duration = Some(duration.as_seconds_f32() as i64);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_running_session() -> FocusSession<RunningSession> {
        FocusSession::<NewSession>::new(Uuid::new_v4(), None, FocusSessionType::Work)
            .unwrap()
            .run_session()
    }

    fn make_terminable_session() -> FocusSession<RunningSession> {
        let session = make_running_session();
        std::thread::sleep(std::time::Duration::from_secs(1));
        session
    }

    #[test]
    fn test_new_session() {
        let user_id = Uuid::new_v4();
        let task_id = Uuid::new_v4();

        let result =
            FocusSession::<NewSession>::new(user_id, Some(task_id), FocusSessionType::Work);

        assert!(result.is_ok());
    }

    #[test]
    fn test_new_session_without_optional_fields() {
        let result =
            FocusSession::<NewSession>::new(Uuid::new_v4(), None, FocusSessionType::ShortBreak);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_session_preserves_user_id() {
        let user_id = Uuid::new_v4();
        let running = FocusSession::<NewSession>::new(user_id, None, FocusSessionType::Work)
            .unwrap()
            .run_session();
        assert_eq!(running.user_id(), user_id);
    }

    #[test]
    fn test_run_session_preserves_session_type() {
        let running =
            FocusSession::<NewSession>::new(Uuid::new_v4(), None, FocusSessionType::LongBreak)
                .unwrap()
                .run_session();
        assert_eq!(running.session_type(), FocusSessionType::LongBreak);
    }

    #[test]
    fn test_update_note() {
        let mut running = make_running_session();
        running.update_note("focus on feature X".to_string());
    }

    #[test]
    fn test_update_concentration_score_boundary_values() {
        let mut running = make_running_session();
        assert!(running.update_concentration_score(0).is_ok());
        assert!(running.update_concentration_score(5).is_ok());
    }

    #[test]
    fn test_update_concentration_score_mid_value() {
        let mut running = make_running_session();
        assert!(running.update_concentration_score(3).is_ok());
    }

    #[test]
    fn test_update_concentration_score_negative_fails() {
        let mut running = make_running_session();
        assert_eq!(
            running.update_concentration_score(-1),
            Err(FocusSessionError::InvalidConcentrationScore(-1))
        );
    }

    #[test]
    fn test_update_concentration_score_above_max_fails() {
        let mut running = make_running_session();
        assert_eq!(
            running.update_concentration_score(6),
            Err(FocusSessionError::InvalidConcentrationScore(6))
        );
    }

    #[test]
    fn test_terminate_ok() {
        let running = make_terminable_session();
        assert!(running.terminate().is_ok());
    }

    #[test]
    fn test_terminate_preserves_user_id() {
        let user_id = Uuid::new_v4();
        let running = FocusSession::<NewSession>::new(user_id, None, FocusSessionType::Work)
            .unwrap()
            .run_session();
        std::thread::sleep(std::time::Duration::from_secs(1));
        let terminated = running.terminate().unwrap();
        assert_eq!(terminated.user_id(), user_id);
    }

    #[test]
    fn test_terminate_preserves_session_type() {
        let running =
            FocusSession::<NewSession>::new(Uuid::new_v4(), None, FocusSessionType::ShortBreak)
                .unwrap()
                .run_session();
        std::thread::sleep(std::time::Duration::from_secs(1));
        let terminated = running.terminate().unwrap();
        assert_eq!(terminated.session_type(), FocusSessionType::ShortBreak);
    }
}
