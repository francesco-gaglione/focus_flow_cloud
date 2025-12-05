use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UpdateFocusSessionData {
    pub session_id: Uuid,
    pub category_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
    pub concentration_score: Option<i32>,
    pub actual_duration: Option<i64>,
    pub notes: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
}
