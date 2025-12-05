use chrono::{DateTime, Utc};
use uuid::Uuid;

use domain::entities::focus_session_type::FocusSessionType;

#[derive(Debug, Clone)]
pub struct FindSessionByFiltersData {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub category_ids: Option<Vec<Uuid>>,
    pub session_type: Option<FocusSessionType>,
    pub min_concentration_score: Option<i32>,
    pub max_concentration_score: Option<i32>,
}
