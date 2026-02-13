use crate::use_cases::task::command::create_task::CreateTaskCommand;

use crate::persistence_traits::persistence_error::PersistenceError;
use crate::persistence_traits::task_persistence::TaskPersistence;
use domain::entities::task::Task;
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error, PartialEq)]
pub enum CreateTaskError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type CreateTaskResult<T> = Result<T, CreateTaskError>;

pub struct CreateTaskUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl CreateTaskUseCase {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    pub async fn execute(&self, command: CreateTaskCommand) -> CreateTaskResult<Uuid> {
        let task = Task::create(
            command.user_id,
            command.category_id,
            command.name.clone(),
            command.description.clone(),
            command.scheduled_date,
        );

        let result = self.task_persistence.create_task(task).await;

        Ok(result?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence_traits::task_persistence::MockTaskPersistence;

    #[tokio::test]
    async fn test_create_task_success() {
        let mut mock_persistence = MockTaskPersistence::new();
        let expected_uuid = Uuid::new_v4();

        mock_persistence
            .expect_create_task()
            .returning(move |_| Ok(expected_uuid));

        let use_case = CreateTaskUseCase::new(Arc::new(mock_persistence));
        let command = CreateTaskCommand {
            user_id: Uuid::new_v4(),
            category_id: None,
            name: "New Task".to_string(),
            description: None,
            scheduled_date: None,
        };

        let result = use_case.execute(command).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_uuid);
    }
}
