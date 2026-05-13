use crate::{
    entities::tasks::{task::Task, task_priority::TaskPriority},
    value_objects::stats::completed_by_priority::CompletedByPriority,
};

pub struct CompletedByPriorityService {}

impl CompletedByPriorityService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn calculate(tasks: &[Task]) -> CompletedByPriority {
        let mut result = CompletedByPriority::new();

        for priority in [
            TaskPriority::Low,
            TaskPriority::Medium,
            TaskPriority::High,
            TaskPriority::Urgent,
        ] {
            let count = tasks
                .iter()
                .filter(|t| t.completed_at().is_some() && t.priority() == Some(priority))
                .count();
            result.update_count(priority, count);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::entities::tasks::{task::Task, task_priority::TaskPriority, task_schedule::TaskSchedule};

    use super::*;

    fn completed_task(priority: Option<TaskPriority>) -> Task {
        let mut t = Task::reconstitute(
            Uuid::new_v4(), Uuid::new_v4(), "task".to_string(), None,
            priority, None, vec![], TaskSchedule::Unscheduled, None, vec![],
        );
        t.complete().unwrap();
        t
    }

    fn pending_task(priority: Option<TaskPriority>) -> Task {
        Task::reconstitute(
            Uuid::new_v4(), Uuid::new_v4(), "task".to_string(), None,
            priority, None, vec![], TaskSchedule::Unscheduled, None, vec![],
        )
    }

    #[test]
    fn test_empty() {
        let result = CompletedByPriorityService::calculate(&[]);
        assert_eq!(result.get_count(TaskPriority::Low), 0);
        assert_eq!(result.get_count(TaskPriority::Medium), 0);
        assert_eq!(result.get_count(TaskPriority::High), 0);
        assert_eq!(result.get_count(TaskPriority::Urgent), 0);
    }

    #[test]
    fn test_counts_by_priority() {
        let tasks = vec![
            completed_task(Some(TaskPriority::Low)),
            completed_task(Some(TaskPriority::Low)),
            completed_task(Some(TaskPriority::High)),
            completed_task(Some(TaskPriority::Urgent)),
        ];
        let result = CompletedByPriorityService::calculate(&tasks);
        assert_eq!(result.get_count(TaskPriority::Low), 2);
        assert_eq!(result.get_count(TaskPriority::Medium), 0);
        assert_eq!(result.get_count(TaskPriority::High), 1);
        assert_eq!(result.get_count(TaskPriority::Urgent), 1);
    }

    #[test]
    fn test_pending_tasks_not_counted() {
        let tasks = vec![
            pending_task(Some(TaskPriority::High)),
            completed_task(Some(TaskPriority::High)),
        ];
        let result = CompletedByPriorityService::calculate(&tasks);
        assert_eq!(result.get_count(TaskPriority::High), 1);
    }

    #[test]
    fn test_no_priority_not_counted() {
        let tasks = vec![completed_task(None)];
        let result = CompletedByPriorityService::calculate(&tasks);
        assert_eq!(result.get_count(TaskPriority::Low), 0);
        assert_eq!(result.get_count(TaskPriority::Medium), 0);
        assert_eq!(result.get_count(TaskPriority::High), 0);
        assert_eq!(result.get_count(TaskPriority::Urgent), 0);
    }
}
