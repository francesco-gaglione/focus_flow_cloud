use crate::persistence_traits::persistence_error::PersistenceError;
use crate::persistence_traits::task_persistence::TaskPersistence;
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error, PartialEq)]
pub enum DeleteTasksError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type DeleteTasksResult<T> = Result<T, DeleteTasksError>;

pub struct DeleteTasksUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl DeleteTasksUseCase {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    pub async fn execute(&self, task_ids: Vec<Uuid>) -> DeleteTasksResult<Vec<Uuid>> {
        let mut deleted_ids = Vec::new();
        for task_id in task_ids {
            self.task_persistence.delete_task(task_id).await?;
            deleted_ids.push(task_id);
        }
        Ok(deleted_ids)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence_traits::task_persistence::MockTaskPersistence;

    #[tokio::test]
    async fn test_delete_tasks_success() {
        let mut mock_persistence = MockTaskPersistence::new();
        let task_ids = vec![uuid::Uuid::new_v4(), uuid::Uuid::new_v4()];
        let expected_deleted_ids = task_ids.clone();

        for &id in &task_ids {
            mock_persistence
                .expect_delete_task()
                .with(mockall::predicate::eq(id))
                .returning(|_| Ok(()));
        }

        let use_case = DeleteTasksUseCase::new(Arc::new(mock_persistence));
        let result = use_case.execute(task_ids).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_deleted_ids);
    }

    #[tokio::test]
    async fn test_delete_tasks_partial_failure() {
        let mut mock_persistence = MockTaskPersistence::new();
        let id1 = uuid::Uuid::new_v4();
        let id2 = uuid::Uuid::new_v4();
        let task_ids = vec![id1, id2];

        mock_persistence
            .expect_delete_task()
            .with(mockall::predicate::eq(id1))
            .returning(|_| Ok(()));

        mock_persistence
            .expect_delete_task()
            .with(mockall::predicate::eq(id2))
            .returning(|_| Err(PersistenceError::Unexpected("Error".to_string())));

        let use_case = DeleteTasksUseCase::new(Arc::new(mock_persistence));
        let result = use_case.execute(task_ids).await;

        assert!(result.is_err());
    }
}
