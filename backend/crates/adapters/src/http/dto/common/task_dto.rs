pub use shared::task::TaskDto;

use application::use_cases::task::get_tasks::TaskOutput;
use domain::entities::tasks::task_priority::TaskPriority;
use shared::task::{SubtaskDto, TaskPriority as SharedTaskPriority};

fn priority_to_dto(p: Option<TaskPriority>) -> Option<SharedTaskPriority> {
    p.map(|p| match p {
        TaskPriority::Low => SharedTaskPriority::Low,
        TaskPriority::Medium => SharedTaskPriority::Medium,
        TaskPriority::High => SharedTaskPriority::High,
        TaskPriority::Urgent => SharedTaskPriority::Urgent,
    })
}

pub fn from_task_output(v: &TaskOutput) -> TaskDto {
    TaskDto {
        id: v.id.to_string(),
        title: v.title.clone(),
        description: v.description.clone(),
        priority: priority_to_dto(v.priority),
        due_date: v.due_date.map(|d| d.timestamp()),
        completed_at: v.completed_at.map(|c| c.timestamp()),
        subtasks: v
            .subtasks
            .iter()
            .map(|s| SubtaskDto {
                id: s.id.to_string(),
                title: s.title.clone(),
                description: s.description.clone(),
                is_completed: s.is_completed,
                sort_order: s.sort_order,
            })
            .collect(),
        category_id: v.category_id.clone(),
    }
}
