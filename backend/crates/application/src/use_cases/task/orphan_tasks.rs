use crate::persistence_traits::persistence_error::PersistenceError;
use crate::persistence_traits::task_persistence::TaskPersistence;
use domain::entities::task::Task;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum OrphanTasksError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type OrphanTasksResult<T> = Result<T, OrphanTasksError>;

pub struct GetOrphanTasksCommand {
    pub completed: Option<bool>,
}

pub struct OrphanTasksUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl OrphanTasksUseCase {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    pub async fn execute(&self, command: GetOrphanTasksCommand) -> OrphanTasksResult<Vec<Task>> {
        Ok(self
            .task_persistence
            .find_orphan_tasks(command.completed.unwrap_or(false))
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence_traits::task_persistence::MockTaskPersistence;

    #[tokio::test]
    async fn test_orphan_tasks_success() {
        let mut mock_persistence = MockTaskPersistence::new();
        let expected_tasks = vec![Task::reconstitute(
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(),
            None,
            "Orphan Task".to_string(),
            None,
            None,
            None,
        )];
        let returned_tasks = expected_tasks.clone();

        mock_persistence
            .expect_find_orphan_tasks()
            .with(mockall::predicate::eq(false))
            .returning(move |_| Ok(returned_tasks.clone()));

        let use_case = OrphanTasksUseCase::new(Arc::new(mock_persistence));
        let command = GetOrphanTasksCommand { completed: None };
        let result = use_case.execute(command).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_tasks);
    }

    #[tokio::test]
    async fn test_orphan_tasks_completed() {
        let mut mock_persistence = MockTaskPersistence::new();
        let expected_tasks = vec![];

        mock_persistence
            .expect_find_orphan_tasks()
            .with(mockall::predicate::eq(true))
            .returning(move |_| Ok(vec![]));

        let use_case = OrphanTasksUseCase::new(Arc::new(mock_persistence));
        let command = GetOrphanTasksCommand {
            completed: Some(true),
        };
        let result = use_case.execute(command).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_tasks);
    }
}
