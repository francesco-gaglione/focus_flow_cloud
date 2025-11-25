use crate::domain::entities::focus_session_type::FocusSessionType;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct FocusSession {
    id: Uuid,
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
            id: Uuid::new_v4(),
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

    #[allow(clippy::too_many_arguments)]
    pub fn new_with_id(
        id: Uuid,
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

    pub fn id(&self) -> Uuid {
        self.id
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
