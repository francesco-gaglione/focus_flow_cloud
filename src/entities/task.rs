use crate::db_models::db_task::DbTask;
use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Task {
    pub id: Uuid,
    pub category_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub scheduled_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl From<&DbTask> for Task {
    fn from(value: &DbTask) -> Self {
        Self {
            id: value.id,
            category_id: value.category_id,
            name: value.name.clone(),
            description: value.description.clone(),
            scheduled_date: value.scheduled_date,
            created_at: value.created_at,
            completed_at: value.completed_at,
        }
    }
}
