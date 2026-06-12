use crate::tasks::traits::task_persistence::TaskPersistence;
use crate::{
    shared::traits::persistence_error::PersistenceError,
    tasks::use_cases::task::common::task_schedule_app_dto::TaskScheduleAppDto,
};
use domain::tasks::entities::task_priority::TaskPriority;
use std::sync::Arc;
use thiserror::Error;
use tracing::{error, info, instrument};
use uuid::Uuid;

#[derive(Debug, Error, PartialEq)]
pub enum UpdateTaskError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),

    #[error("Sub-tasks must be completed before marking task as completed")]
    UncompletedSubTasks,
}

pub type UpdateTaskResult<T> = Result<T, UpdateTaskError>;

#[derive(Debug, Clone)]
pub struct UpdateTaskCommand {
    pub id: Uuid,
    pub title: Option<String>,
    pub description: Option<String>,
    pub schedule: Option<TaskScheduleAppDto>,
    pub priority: Option<TaskPriority>,
    pub completed: Option<bool>,
}

pub struct UpdateTaskUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl UpdateTaskUseCase {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, command: UpdateTaskCommand) -> UpdateTaskResult<()> {
        let mut task = self.task_persistence.find_by_id(command.id).await?;

        if let Some(title) = command.title {
            info!("Updating title to: {}", title);
            task.update_title(title);
        }
        if let Some(description) = command.description {
            info!("Updating description to: {}", description);
            task.update_description(Some(description));
        }
        if let Some(schedule) = command.schedule {
            info!("Updating task schedule to: {:?}", schedule);
            task.update_schedule(schedule.into());
        }
        if let Some(priority) = command.priority {
            info!("Updating priority to: {:?}", priority);
            task.update_priority(Some(priority));
        }
        if let Some(completed) = command.completed {
            info!("Updating completed status to: {}", completed);
            if completed {
                task.complete().map_err(|_| {
                    error!("Uncompleted sub-tasks");
                    UpdateTaskError::UncompletedSubTasks
                })?;
            } else {
                task.uncomplete();
            }
        }

        info!("Updating task: {}", command.id);
        self.task_persistence.update_task(task).await?;
        info!("Task updated successfully");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tasks::traits::task_persistence::MockTaskPersistence;
    use domain::tasks::entities::{task::Task, task_schedule::TaskSchedule};

    #[tokio::test]
    async fn test_update_task_success() {
        let mut mock_persistence = MockTaskPersistence::new();
        let task_id = Uuid::new_v4();
        let task = Task::new(
            task_id,
            "Old Title".to_string(),
            TaskSchedule::Unscheduled,
            None,
        );
        let task_to_return = task.clone();

        mock_persistence
            .expect_find_by_id()
            .with(mockall::predicate::eq(task_id))
            .returning(move |_| Ok(task_to_return.clone()));

        mock_persistence
            .expect_update_task()
            .returning(|task| Ok(task));

        let use_case = UpdateTaskUseCase::new(Arc::new(mock_persistence));
        let command = UpdateTaskCommand {
            id: task_id,
            title: Some("New Title".to_string()),
            description: None,
            schedule: None,
            priority: None,
            completed: None,
        };

        let result = use_case.execute(command).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_task_not_found() {
        let mut mock_persistence = MockTaskPersistence::new();
        mock_persistence
            .expect_find_by_id()
            .returning(|_| Err(PersistenceError::Unexpected("Not found".to_string())));

        let use_case = UpdateTaskUseCase::new(Arc::new(mock_persistence));
        let command = UpdateTaskCommand {
            id: Uuid::new_v4(),
            title: Some("New Title".to_string()),
            description: None,
            schedule: None,
            priority: None,
            completed: None,
        };

        let result = use_case.execute(command).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_task_priority() {
        let mut mock_persistence = MockTaskPersistence::new();
        let task_id = Uuid::new_v4();
        let task = Task::new(task_id, "Task".to_string(), TaskSchedule::Unscheduled, None);
        let task_to_return = task.clone();

        mock_persistence
            .expect_find_by_id()
            .returning(move |_| Ok(task_to_return.clone()));

        mock_persistence
            .expect_update_task()
            .withf(|t| t.priority() == Some(TaskPriority::High))
            .returning(|task| Ok(task));

        let use_case = UpdateTaskUseCase::new(Arc::new(mock_persistence));
        let command = UpdateTaskCommand {
            id: task_id,
            title: None,
            description: None,
            schedule: None,
            priority: Some(TaskPriority::High),
            completed: None,
        };

        let result = use_case.execute(command).await;

        assert!(result.is_ok());
    }
}
