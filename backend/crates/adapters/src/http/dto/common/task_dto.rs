pub use shared::task::TaskDto;

use application::use_cases::{
    category::get_category_and_task_usecase,
    task::{get_tasks::TaskOutput, orphan_tasks},
};
use domain::entities::tasks::task_priority::TaskPriority;
use shared::task::TaskPriority as SharedTaskPriority;

fn priority_to_dto(p: Option<TaskPriority>) -> Option<SharedTaskPriority> {
    p.map(|p| match p {
        TaskPriority::Low => SharedTaskPriority::Low,
        TaskPriority::Medium => SharedTaskPriority::Medium,
        TaskPriority::High => SharedTaskPriority::High,
        TaskPriority::Urgent => SharedTaskPriority::Urgent,
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
        priority: priority_to_dto(v.priority),
        due_date: v.due_date.map(|d| d.timestamp()),
        completed_at: v.completed_at.map(|c| c.timestamp()),
    }
}

pub fn from_task_output(v: &TaskOutput) -> TaskDto {
    TaskDto {
        id: v.id.to_string(),
        title: v.title.clone(),
        description: v.description.clone(),
        priority: priority_to_dto(v.priority),
        due_date: v.due_date.map(|d| d.timestamp()),
        completed_at: v.completed_at.map(|c| c.timestamp()),
    }
}
