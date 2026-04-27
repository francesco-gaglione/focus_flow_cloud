use application::use_cases::{
    category::get_category_and_task_usecase,
    task::{get_tasks::TaskOutput, orphan_tasks},
};
use domain::entities::tasks::task_priority::TaskPriority;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TaskDto {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub due_date: Option<i64>,      // timestamp in seconds
    pub completed_at: Option<i64>,  // timestamp in seconds
}

fn priority_to_str(p: Option<TaskPriority>) -> Option<String> {
    p.map(|p| match p {
        TaskPriority::Low => "low".to_string(),
        TaskPriority::Medium => "medium".to_string(),
        TaskPriority::High => "high".to_string(),
        TaskPriority::Urgent => "urgent".to_string(),
    })
}

impl From<&get_category_and_task_usecase::TaskDto> for TaskDto {
    fn from(value: &get_category_and_task_usecase::TaskDto) -> Self {
        Self {
            id: value.id.to_string(),
            title: value.title.clone(),
            description: value.description.clone(),
            priority: None,
            due_date: value.due_date.map(|d| d.timestamp()),
            completed_at: value.completed_at.map(|c| c.timestamp()),
        }
    }
}

impl From<&orphan_tasks::TaskOutput> for TaskDto {
    fn from(value: &orphan_tasks::TaskOutput) -> Self {
        Self {
            id: value.id.to_string(),
            title: value.title.clone(),
            description: value.description.clone(),
            priority: priority_to_str(value.priority),
            due_date: value.due_date.map(|d| d.timestamp()),
            completed_at: value.completed_at.map(|c| c.timestamp()),
        }
    }
}

impl From<&TaskOutput> for TaskDto {
    fn from(value: &TaskOutput) -> Self {
        Self {
            id: value.id.to_string(),
            title: value.title.clone(),
            description: value.description.clone(),
            priority: priority_to_str(value.priority),
            due_date: value.due_date.map(|d| d.timestamp()),
            completed_at: value.completed_at.map(|c| c.timestamp()),
        }
    }
}
