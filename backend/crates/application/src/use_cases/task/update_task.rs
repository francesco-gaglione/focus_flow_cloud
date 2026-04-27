use crate::repository_traits::persistence_error::PersistenceError;
use crate::repository_traits::task_persistence::TaskPersistence;
use chrono::{DateTime, Utc};
use domain::entities::tasks::task_priority::TaskPriority;
use std::sync::Arc;
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug, Error, PartialEq)]
pub enum UpdateTaskError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type UpdateTaskResult<T> = Result<T, UpdateTaskError>;

#[derive(Debug, Clone)]
pub struct UpdateTaskCommand {
    pub id: Uuid,
    pub title: Option<String>,
    pub description: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub priority: Option<TaskPriority>,
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
            task.update_title(title);
        }
        if let Some(description) = command.description {
            task.update_description(Some(description));
        }
        if let Some(due_date) = command.due_date {
            task.update_due_date(Some(due_date));
        }
        if let Some(priority) = command.priority {
            task.update_priority(Some(priority));
        }

        self.task_persistence.update_task(task).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository_traits::task_persistence::MockTaskPersistence;
    use domain::entities::tasks::task::Task;

    #[tokio::test]
    async fn test_update_task_success() {
        let mut mock_persistence = MockTaskPersistence::new();
        let task_id = Uuid::new_v4();
        let task = Task::new(task_id, "Old Title".to_string(), None, None);
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
            due_date: None,
            priority: None,
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
            due_date: None,
            priority: None,
        };

        let result = use_case.execute(command).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_task_priority() {
        let mut mock_persistence = MockTaskPersistence::new();
        let task_id = Uuid::new_v4();
        let task = Task::new(task_id, "Task".to_string(), None, None);
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
            due_date: None,
            priority: Some(TaskPriority::High),
        };

        let result = use_case.execute(command).await;

        assert!(result.is_ok());
    }
}
