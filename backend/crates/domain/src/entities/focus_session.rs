use crate::entities::focus_session_type::FocusSessionType;
use chrono::{DateTime, TimeDelta, Utc};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error, PartialEq)]
pub enum FocusSessionError {
    #[error("Invalid focus session parameter: {0}")]
    InvalidFocusSessionParam(String),

    #[error("Invalid focus session duration: {0}")]
    InvalidFocusSessionDuration(TimeDelta),
}

pub type FocusSessionResult<T> = Result<T, FocusSessionError>;

#[derive(Debug, Clone)]
pub struct FocusSession {
    id: Uuid,
    user_id: Uuid,
    category_id: Option<Uuid>,
    task_id: Option<Uuid>,
    session_type: FocusSessionType,
    actual_duration: Option<i64>,
    concentration_score: Option<i32>,
    notes: Option<String>,
    started_at: DateTime<Utc>,
    ended_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
}

impl FocusSession {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        user_id: Uuid,
        category_id: Option<Uuid>,
        task_id: Option<Uuid>,
        session_type: FocusSessionType,
        concentration_score: Option<i32>,
        notes: Option<String>,
        started_at: DateTime<Utc>,
        ended_at: Option<DateTime<Utc>>,
    ) -> FocusSessionResult<Self> {
        if matches!(
            session_type,
            FocusSessionType::ShortBreak | FocusSessionType::LongBreak
        ) && concentration_score.is_some()
        {
            return Err(FocusSessionError::InvalidFocusSessionParam(
                "concentration_score".to_string(),
            ));
        }

        if notes.is_some() && notes.as_ref().unwrap().is_empty() {
            return Err(FocusSessionError::InvalidFocusSessionParam(
                "Notes field is empty".to_string(),
            ));
        }

        let actual_duration = ended_at.map(|ended_at| (ended_at - started_at).num_seconds());

        Ok(FocusSession {
            id: Uuid::new_v4(),
            user_id,
            category_id,
            task_id,
            session_type,
            actual_duration,
            concentration_score,
            notes,
            started_at,
            ended_at,
            created_at: Utc::now(),
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: Uuid,
        user_id: Uuid,
        category_id: Option<Uuid>,
        task_id: Option<Uuid>,
        session_type: FocusSessionType,
        actual_duration: Option<i64>,
        concentration_score: Option<i32>,
        notes: Option<String>,
        started_at: DateTime<Utc>,
        ended_at: Option<DateTime<Utc>>,
        created_at: DateTime<Utc>,
    ) -> Self {
        FocusSession {
            id,
            user_id,
            category_id,
            task_id,
            session_type,
            actual_duration,
            concentration_score,
            notes,
            started_at,
            ended_at,
            created_at,
        }
    }

    pub fn update_category_id(&mut self, category_id: Option<Uuid>) {
        self.category_id = category_id;
    }

    pub fn update_task_id(&mut self, task_id: Option<Uuid>) {
        self.task_id = task_id;
    }

    pub fn update_session_type(&mut self, session_type: FocusSessionType) {
        self.session_type = session_type;
    }

    pub fn update_concentration_score(&mut self, concentration_score: Option<i32>) {
        self.concentration_score = concentration_score;
    }

    pub fn update_notes(&mut self, notes: Option<String>) {
        self.notes = notes;
    }

    pub fn update_date_range(
        &mut self,
        started_at: DateTime<Utc>,
        ended_at: Option<DateTime<Utc>>,
    ) -> FocusSessionResult<()> {
        if let Some(ended_at) = ended_at {
            if ended_at < started_at {
                return Err(FocusSessionError::InvalidFocusSessionDuration(
                    ended_at - started_at,
                ));
            }
        }
        self.started_at = started_at;
        self.ended_at = ended_at;
        self.actual_duration = ended_at
            .map(|ended_at| ended_at - started_at)
            .map(|duration| duration.num_seconds());
        Ok(())
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn category_id(&self) -> Option<Uuid> {
        self.category_id
    }

    pub fn task_id(&self) -> Option<Uuid> {
        self.task_id
    }

    pub fn session_type(&self) -> FocusSessionType {
        self.session_type
    }

    pub fn actual_duration(&self) -> Option<i64> {
        self.actual_duration
    }

    pub fn concentration_score(&self) -> Option<i32> {
        self.concentration_score
    }

    pub fn notes(&self) -> Option<String> {
        self.notes.clone()
    }

    pub fn started_at(&self) -> DateTime<Utc> {
        self.started_at
    }

    pub fn ended_at(&self) -> Option<DateTime<Utc>> {
        self.ended_at
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

#[derive(Debug, Clone)]
pub struct SessionFilter {
    pub user_id: Uuid,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub category_ids: Option<Vec<Uuid>>,
    pub task_ids: Option<Vec<Uuid>>,
    pub session_type: Option<FocusSessionType>,
    pub min_concentration_score: Option<i32>,
    pub max_concentration_score: Option<i32>,
    pub has_notes: Option<bool>,
}
