pub use shared::task::TaskDto;

use application::use_cases::{
    category::get_category_and_task_usecase,
    task::{get_tasks::TaskOutput, orphan_tasks},
};
use domain::entities::tasks::task_priority::TaskPriority;

fn priority_to_str(p: Option<TaskPriority>) -> Option<String> {
    p.map(|p| match p {
        TaskPriority::Low => "low".to_string(),
        TaskPriority::Medium => "medium".to_string(),
        TaskPriority::High => "high".to_string(),
        TaskPriority::Urgent => "urgent".to_string(),
    })
}

pub fn from_category_task(v: &get_category_and_task_usecase::TaskDto) -> TaskDto {
    TaskDto {
        id: v.id.to_string(),
        title: v.title.clone(),
        description: v.description.clone(),
        priority: None,
        due_date: v.due_date.map(|d| d.timestamp()),
        completed_at: v.completed_at.map(|c| c.timestamp()),
    }
}

pub fn from_orphan_task(v: &orphan_tasks::TaskOutput) -> TaskDto {
    TaskDto {
        id: v.id.to_string(),
        title: v.title.clone(),
        description: v.description.clone(),
        priority: priority_to_str(v.priority),
        due_date: v.due_date.map(|d| d.timestamp()),
        completed_at: v.completed_at.map(|c| c.timestamp()),
    }
}

pub fn from_task_output(v: &TaskOutput) -> TaskDto {
    TaskDto {
        id: v.id.to_string(),
        title: v.title.clone(),
        description: v.description.clone(),
        priority: priority_to_str(v.priority),
        due_date: v.due_date.map(|d| d.timestamp()),
        completed_at: v.completed_at.map(|c| c.timestamp()),
    }
}
