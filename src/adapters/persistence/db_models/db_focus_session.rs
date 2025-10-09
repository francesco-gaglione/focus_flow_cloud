use crate::domain::entities::focus_session::FocusSession;
use crate::domain::entities::focus_session_type::FocusSessionType;
use crate::{
    adapters::schema,
    application::use_cases::persistance_command::create_manual_session_data::CreateManualSessionData,
};
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::focus_session)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbFocusSession {
    pub id: Uuid,
    pub task_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub session_type: String,
    pub actual_duration_minutes: Option<i64>,
    pub concentration_score: Option<i32>,
    pub notes: Option<String>,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::focus_session)]
pub struct NewDbFocusSession {
    pub task_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub session_type: String,
    pub concentration_score: Option<i32>,
    pub notes: Option<String>,
    pub actual_duration_minutes: Option<i64>,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = schema::focus_session)]
pub struct UpdateDbFocusSession {
    pub actual_duration_minutes: Option<i64>,
    pub concentration_score: Option<i32>,
    pub notes: Option<String>,
    pub ended_at: Option<DateTime<Utc>>,
}

impl From<DbFocusSession> for FocusSession {
    fn from(value: DbFocusSession) -> Self {
        Self {
            id: value.id,
            category_id: value.category_id,
            task_id: value.task_id,
            session_type: FocusSessionType::from_str(&value.session_type).unwrap(),
            actual_duration_minutes: value.actual_duration_minutes,
            concentration_score: value.concentration_score,
            notes: value.notes,
            started_at: value.started_at,
            ended_at: value.ended_at,
            created_at: value.created_at,
        }
    }
}

impl From<CreateManualSessionData> for NewDbFocusSession {
    fn from(value: CreateManualSessionData) -> Self {
        Self {
            task_id: value.task_id,
            category_id: value.category_id,
            session_type: value.session_type.to_string(),
            concentration_score: value.concentration_score,
            notes: value.notes,
            actual_duration_minutes: Some(value.actual_duration_minutes),
            started_at: value.started_at,
            ended_at: Some(value.ended_at),
        }
    }
}
