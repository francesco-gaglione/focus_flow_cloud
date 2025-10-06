use crate::adapters::schema;
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
    pub category_id: Uuid,
    pub session_type: String,
    pub planned_duration_minutes: i32,
    pub actual_duration_minutes: Option<i32>,
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
    pub category_id: Uuid,
    pub session_type: String,
    pub planned_duration_minutes: i32,
    pub concentration_score: Option<i32>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = schema::focus_session)]
pub struct UpdateDbFocusSession {
    pub actual_duration_minutes: Option<i32>,
    pub concentration_score: Option<i32>,
    pub notes: Option<String>,
    pub ended_at: Option<DateTime<Utc>>,
}
