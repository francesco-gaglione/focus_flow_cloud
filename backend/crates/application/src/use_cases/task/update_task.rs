use crate::persistence_traits::persistence_error::PersistenceError;
use crate::persistence_traits::task_persistence::TaskPersistence;
use crate::use_cases::task::command::update_task::UpdateTaskCommand;
use domain::entities::task::Task;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum UpdateTaskError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type UpdateTaskResult<T> = Result<T, UpdateTaskError>;

pub struct UpdateTaskUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl UpdateTaskUseCase {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    pub async fn execute(&self, command: UpdateTaskCommand) -> UpdateTaskResult<Task> {
        let mut task = self.task_persistence.find_by_id(command.id).await?;

        if let Some(name) = command.name {
            task.update_name(name);
        }
        if let Some(category_id) = command.category_id {
            task.update_category(Some(category_id));
        }
        if let Some(description) = command.description {
            task.update_description(Some(description));
        }
        if let Some(scheduled_date) = command.scheduled_date {
            task.update_scheduled_date(Some(scheduled_date));
        }

        if let Some(completed_at) = command.completed_at {
            task.update_completed_at(Some(completed_at));
        }

        Ok(self.task_persistence.update_task(task).await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence_traits::task_persistence::MockTaskPersistence;

    #[tokio::test]
    async fn test_update_task_success() {
        let mut mock_persistence = MockTaskPersistence::new();
        let task_id = uuid::Uuid::new_v4();
        let original_task = Task::reconstitute(
            task_id.clone(),
            uuid::Uuid::new_v4(),
            Some(uuid::Uuid::new_v4()),
            "Old Name".to_string(),
            Some("Old Desc".to_string()),
            None,
            None,
        );
        let task_to_return = original_task.clone();

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
            name: Some("New Name".to_string()),
            category_id: None,
            description: None,
            scheduled_date: None,
            completed_at: None,
        };

        let result = use_case.execute(command).await;

        assert!(result.is_ok());
        let updated_task = result.unwrap();
        assert_eq!(updated_task.name(), "New Name");
    }

    #[tokio::test]
    async fn test_update_task_not_found() {
        let mut mock_persistence = MockTaskPersistence::new();
        mock_persistence
            .expect_find_by_id()
            .returning(|_| Err(PersistenceError::Unexpected("Not found".to_string())));

        let use_case = UpdateTaskUseCase::new(Arc::new(mock_persistence));
        let command = UpdateTaskCommand {
            id: uuid::Uuid::new_v4(),
            name: Some("New Name".to_string()),
            category_id: None,
            description: None,
            scheduled_date: None,
            completed_at: None,
        };

        let result = use_case.execute(command).await;

        assert!(result.is_err());
    }
}
