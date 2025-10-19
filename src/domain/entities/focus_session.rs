use crate::domain::entities::focus_session_type::FocusSessionType;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct FocusSession {
    pub id: Uuid,
    pub category_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
    pub session_type: FocusSessionType,
    pub actual_duration: Option<i64>,
    pub concentration_score: Option<i32>,
    pub notes: Option<String>,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
