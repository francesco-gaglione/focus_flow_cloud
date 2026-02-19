use std::sync::Arc;

use thiserror::Error;
use uuid::Uuid;

use crate::persistence_traits::{
    persistence_error::PersistenceError, task_persistence::TaskPersistence,
};

#[derive(Debug, Error, PartialEq)]
pub enum CompleteTaskError {
    #[error("Task not found: {0}")]
    TaskNotFound(Uuid),

    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type CompleteTaskResult<T> = Result<T, CompleteTaskError>;

pub struct CompleteTaskCommand {
    pub id: Uuid,
}

pub struct CompleteTaskUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl CompleteTaskUseCase {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    pub async fn execute(&self, command: CompleteTaskCommand) -> CompleteTaskResult<()> {
        let mut task = self.task_persistence.find_by_id(command.id).await?;

        task.complete();

        self.task_persistence.update_task(task).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence_traits::task_persistence::MockTaskPersistence;
    use chrono::Utc;
    use domain::entities::task::Task;

    #[tokio::test]
    async fn test_complete_task_success() {
        let mut mock_persistence = MockTaskPersistence::new();
        let task_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let command = CompleteTaskCommand { id: task_id };

        let task = Task::reconstitute(
            task_id,
            user_id,
            None,
            "Test Task".to_string(),
            None,
            None,
            None,
        );

        mock_persistence
            .expect_find_by_id()
            .with(mockall::predicate::eq(task_id))
            .times(1)
            .returning(move |_| Ok(task.clone()));

        mock_persistence
            .expect_update_task()
            .withf(|t| t.is_completed())
            .times(1)
            .returning(|t| Ok(t));

        let use_case = CompleteTaskUseCase::new(Arc::new(mock_persistence));
        let result = use_case.execute(command).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_complete_task_find_error() {
        let mut mock_persistence = MockTaskPersistence::new();
        let task_id = Uuid::new_v4();
        let command = CompleteTaskCommand { id: task_id };

        mock_persistence
            .expect_find_by_id()
            .with(mockall::predicate::eq(task_id))
            .times(1)
            .returning(|_| Err(PersistenceError::NotFound("Task not found".to_string())));

        let use_case = CompleteTaskUseCase::new(Arc::new(mock_persistence));
        let result = use_case.execute(command).await;

        assert!(matches!(
            result,
            Err(CompleteTaskError::PersistenceError(
                PersistenceError::NotFound(_)
            ))
        ));
    }

    #[tokio::test]
    async fn test_complete_task_update_error() {
        let mut mock_persistence = MockTaskPersistence::new();
        let task_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let command = CompleteTaskCommand { id: task_id };

        let task = Task::reconstitute(
            task_id,
            user_id,
            None,
            "Test Task".to_string(),
            None,
            None,
            None,
        );

        mock_persistence
            .expect_find_by_id()
            .with(mockall::predicate::eq(task_id))
            .times(1)
            .returning(move |_| Ok(task.clone()));

        mock_persistence
            .expect_update_task()
            .times(1)
            .returning(|_| Err(PersistenceError::Unexpected("Update failed".to_string())));

        let use_case = CompleteTaskUseCase::new(Arc::new(mock_persistence));
        let result = use_case.execute(command).await;

        assert!(matches!(
            result,
            Err(CompleteTaskError::PersistenceError(
                PersistenceError::Unexpected(_)
            ))
        ));
    }

    #[tokio::test]
    async fn test_complete_already_completed_task() {
        let mut mock_persistence = MockTaskPersistence::new();
        let task_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let command = CompleteTaskCommand { id: task_id };

        let old_completed_at = Utc::now() - chrono::Duration::hours(1);
        let task = Task::reconstitute(
            task_id,
            user_id,
            None,
            "Already Completed Task".to_string(),
            None,
            None,
            Some(old_completed_at),
        );

        mock_persistence
            .expect_find_by_id()
            .with(mockall::predicate::eq(task_id))
            .times(1)
            .returning(move |_| Ok(task.clone()));

        mock_persistence
            .expect_update_task()
            .withf(move |t| t.is_completed() && t.completed_at().unwrap() > old_completed_at)
            .times(1)
            .returning(|t| Ok(t));

        let use_case = CompleteTaskUseCase::new(Arc::new(mock_persistence));
        let result = use_case.execute(command).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_complete_task_preserves_data() {
        let mut mock_persistence = MockTaskPersistence::new();
        let task_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let category_id = Uuid::new_v4();
        let command = CompleteTaskCommand { id: task_id };

        let name = "Preserved Name".to_string();
        let description = Some("Preserved Description".to_string());

        let task = Task::reconstitute(
            task_id,
            user_id,
            Some(category_id),
            name.clone(),
            description.clone(),
            None,
            None,
        );

        mock_persistence
            .expect_find_by_id()
            .with(mockall::predicate::eq(task_id))
            .times(1)
            .returning(move |_| Ok(task.clone()));

        mock_persistence
            .expect_update_task()
            .withf(move |t| {
                t.id() == task_id
                    && t.user_id() == user_id
                    && t.category_id() == Some(category_id)
                    && t.name() == name
                    && t.description() == description.as_deref()
            })
            .times(1)
            .returning(|t| Ok(t));

        let use_case = CompleteTaskUseCase::new(Arc::new(mock_persistence));
        let result = use_case.execute(command).await;

        assert!(result.is_ok());
    }
}
