use crate::entities::task::Task;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TaskDto {
    pub id: String,
    pub category_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub scheduled_date: Option<i64>, // timestamp in seconds
    pub created_at: i64,             // timestamp in seconds
    pub completed_at: Option<i64>,   // timestamp in seconds
}

impl From<&Task> for TaskDto {
    fn from(value: &Task) -> Self {
        Self {
            id: value.id.to_string(),
            category_id: value.category_id.map(|id| id.to_string()),
            name: value.name.clone(),
            description: value.description.clone(),
            scheduled_date: value.scheduled_date.map(|s| {
                DateTime::<Utc>::from_utc(s.and_hms(0, 0, 0), Utc).timestamp_millis() / 1000
            }),
            created_at: value.created_at.timestamp_millis() / 1000,
            completed_at: value
                .completed_at
                .map(|completed_at| completed_at.timestamp_millis() / 1000),
        }
    }
}
