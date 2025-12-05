use chrono::{DateTime, Utc};
use uuid::Uuid;

use domain::entities::focus_session_type::FocusSessionType;

#[derive(Debug, Clone)]
pub struct CreateSessionData {
    pub task_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub session_type: FocusSessionType,
    pub concentration_score: Option<i32>,
    pub notes: Option<String>,
    pub actual_duration: i64,
    pub started_at: DateTime<Utc>,
    pub ended_at: DateTime<Utc>,
}
