use crate::persistence::schema;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use domain::tasks::entities::focus_session::{FocusSession, TerminatedSession};
use domain::tasks::entities::focus_session_type::FocusSessionType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::focus_session)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbFocusSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub task_id: Option<Uuid>,
    pub session_type: String,
    pub actual_duration: Option<i64>,
    pub concentration_score: Option<i32>,
    pub notes: Option<String>,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::focus_session)]
pub struct NewDbFocusSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub task_id: Option<Uuid>,
    pub session_type: String,
    pub concentration_score: Option<i32>,
    pub notes: Option<String>,
    pub actual_duration: Option<i64>,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = schema::focus_session)]
pub struct UpdateDbFocusSession {
    pub task_id: Option<Uuid>,
    pub actual_duration: Option<i64>,
    pub concentration_score: Option<i32>,
    pub notes: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
}

impl From<DbFocusSession> for FocusSession<TerminatedSession> {
    fn from(value: DbFocusSession) -> Self {
        Self::reconstitute(
            value.id,
            value.user_id,
            value.task_id,
            FocusSessionType::from_str(&value.session_type).unwrap(),
            value.actual_duration,
            value.concentration_score,
            value.notes,
            value.started_at,
            value.ended_at,
        )
    }
}

impl From<FocusSession<TerminatedSession>> for NewDbFocusSession {
    fn from(value: FocusSession<TerminatedSession>) -> Self {
        Self {
            id: value.id(),
            user_id: value.user_id(),
            task_id: value.task_id(),
            session_type: value.session_type().to_string(),
            concentration_score: value.concentration_score(),
            notes: value.note(),
            actual_duration: Some(value.actual_duration()),
            started_at: value.started_at(),
            ended_at: Some(value.ended_at()),
        }
    }
}

impl From<FocusSession<TerminatedSession>> for UpdateDbFocusSession {
    fn from(value: FocusSession<TerminatedSession>) -> Self {
        Self {
            task_id: value.task_id(),
            actual_duration: Some(value.actual_duration()),
            concentration_score: value.concentration_score(),
            notes: value.note(),
            started_at: Some(value.started_at()),
            ended_at: Some(value.ended_at()),
        }
    }
}
