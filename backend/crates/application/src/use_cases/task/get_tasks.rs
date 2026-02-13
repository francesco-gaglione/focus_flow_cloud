use crate::persistence_traits::persistence_error::PersistenceError;
use crate::persistence_traits::task_persistence::TaskPersistence;
use domain::entities::task::Task;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum GetTaskError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type GetTasksResult<T> = Result<T, GetTaskError>;

pub struct GetTasksUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl GetTasksUseCase {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    pub async fn execute(&self) -> GetTasksResult<Vec<Task>> {
        Ok(self.task_persistence.find_all().await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence_traits::task_persistence::MockTaskPersistence;

    #[tokio::test]
    async fn test_get_tasks_success() {
        let mut mock_persistence = MockTaskPersistence::new();
        let expected_tasks = vec![Task::reconstitute(
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(),
            None,
            "Task 1".to_string(),
            None,
            None,
            None,
        )];
        let returned_tasks = expected_tasks.clone();

        mock_persistence
            .expect_find_all()
            .returning(move || Ok(returned_tasks.clone()));

        let use_case = GetTasksUseCase::new(Arc::new(mock_persistence));
        let result = use_case.execute().await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_tasks);
    }
}
