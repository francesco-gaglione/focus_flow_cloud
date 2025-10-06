use crate::domain::entities::category::Category;
use crate::domain::entities::focus_session_type::FocusSessionType;
use crate::domain::entities::task::Task;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct FocusSession {
    pub id: Uuid,
    pub category: Option<Category>,
    pub task: Option<Task>,
    pub session_type: FocusSessionType,
    pub planned_duration_minutes: i32,
    pub actual_duration_minutes: Option<i32>,
    pub concentration_score: Option<i32>,
    pub notes: Option<String>,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
