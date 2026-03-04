use application::use_cases::{
    category::get_category_and_task_usecase, task::get_tasks::TaskOutput,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TaskDto {
    pub id: String,
    pub category_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub scheduled_date: Option<i64>,     // timestamp in seconds
    pub scheduled_end_date: Option<i64>, // timestamp in seconds
    pub completed_at: Option<i64>,       // timestamp in seconds
}

impl From<&get_category_and_task_usecase::TaskDto> for TaskDto {
    fn from(value: &get_category_and_task_usecase::TaskDto) -> Self {
        Self {
            id: value.id.to_string(),
            category_id: value.category_id.map(|c| c.to_string()),
            name: value.name.clone(),
            description: value.description.clone(),
            scheduled_date: value.scheduled_date.map(|s| s.timestamp()),
            scheduled_end_date: value.scheduled_end_date.map(|s| s.timestamp()),
            completed_at: value.completed_at.map(|c| c.timestamp()),
        }
    }
}

impl From<&application::use_cases::task::orphan_tasks::TaskOutput> for TaskDto {
    fn from(value: &application::use_cases::task::orphan_tasks::TaskOutput) -> Self {
        Self {
            id: value.id.to_string(),
            category_id: value.category_id.map(|c| c.to_string()),
            name: value.name.clone(),
            description: value.description.clone(),
            scheduled_date: value.scheduled_date.map(|s| s.timestamp()),
            scheduled_end_date: value.scheduled_end_date.map(|s| s.timestamp()),
            completed_at: value.completed_at.map(|c| c.timestamp()),
        }
    }
}

impl From<&TaskOutput> for TaskDto {
    fn from(value: &TaskOutput) -> Self {
        Self {
            id: value.id.to_string(),
            category_id: value.category_id.map(|id| id.to_string()),
            name: value.name.clone(),
            description: value.description.clone(),
            scheduled_date: value.scheduled_date.map(|d| d.timestamp()),
            scheduled_end_date: value.scheduled_end_date.map(|d| d.timestamp()),
            completed_at: value.completed_at.map(|d| d.timestamp()),
        }
    }
}
