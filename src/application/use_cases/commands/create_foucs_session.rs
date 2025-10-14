use crate::domain::entities::focus_session_type::FocusSessionType;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateFocusSessionCommand {
    pub category_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
    pub session_type: FocusSessionType,
    pub concentration_score: Option<i32>, // if none a default will be used (5)
    pub actual_duration: i64,
    pub notes: Option<String>,
    pub started_at: DateTime<Utc>,
    pub ended_at: DateTime<Utc>,
}
