use crate::entities::task::Task;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TaskDto {
    pub id: String,
    pub category_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub scheduled_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl From<&Task> for TaskDto {
    fn from(value: &Task) -> Self {
        Self {
            id: value.id.to_string(),
            category_id: value.category_id.map(|id| id.to_string()),
            name: value.name.clone(),
            description: value.description.clone(),
            scheduled_date: value.scheduled_date,
            created_at: value.created_at,
            completed_at: value.completed_at,
        }
    }
}
