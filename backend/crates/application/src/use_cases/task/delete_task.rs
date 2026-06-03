use crate::repository_traits::persistence_error::PersistenceError;
use crate::repository_traits::task_persistence::TaskPersistence;
use std::sync::Arc;
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug, Error, PartialEq)]
pub enum DeleteTaskError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type DeleteTaskResult<T> = Result<T, DeleteTaskError>;

pub struct DeleteTaskUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl DeleteTaskUseCase {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, task_id: Uuid) -> DeleteTaskResult<Uuid> {
        self.task_persistence.delete_task(task_id).await?;
        Ok(task_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository_traits::task_persistence::MockTaskPersistence;

    #[tokio::test]
    async fn test_delete_tasks_success() {
        let mut mock_persistence = MockTaskPersistence::new();
        let task_ids = uuid::Uuid::new_v4();
        let expected_deleted_ids = task_ids.clone();

        mock_persistence
            .expect_delete_task()
            .with(mockall::predicate::eq(task_ids))
            .returning(|_| Ok(()));

        let use_case = DeleteTaskUseCase::new(Arc::new(mock_persistence));
        let result = use_case.execute(task_ids).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_deleted_ids);
    }

    #[tokio::test]
    async fn test_delete_tasks_partial_failure() {
        let mut mock_persistence = MockTaskPersistence::new();
        let id1 = uuid::Uuid::new_v4();
        let id2 = uuid::Uuid::new_v4();

        mock_persistence
            .expect_delete_task()
            .with(mockall::predicate::eq(id1))
            .returning(|_| Ok(()));

        mock_persistence
            .expect_delete_task()
            .with(mockall::predicate::eq(id2))
            .returning(|_| Err(PersistenceError::Unexpected("Error".to_string())));

        let use_case = DeleteTaskUseCase::new(Arc::new(mock_persistence));
        let _ = use_case.execute(id1).await;
        let result = use_case.execute(id2).await;

        assert!(result.is_err());
    }
}
