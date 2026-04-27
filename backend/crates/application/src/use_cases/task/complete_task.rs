use std::sync::Arc;

use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

use crate::repository_traits::{
    persistence_error::PersistenceError, task_persistence::TaskPersistence,
};

#[derive(Debug, Error, PartialEq)]
pub enum CompleteTaskError {
    #[error("Task not found: {0}")]
    TaskNotFound(Uuid),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Sub-tasks must be completed before marking task as completed")]
    UncompletedSubTasks,

    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type CompleteTaskResult<T> = Result<T, CompleteTaskError>;

#[derive(Debug)]
pub struct CompleteTaskCommand {
    pub id: Uuid,
    pub user_id: Uuid,
}

pub struct CompleteTaskUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl CompleteTaskUseCase {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, command: CompleteTaskCommand) -> CompleteTaskResult<()> {
        let mut task = self.task_persistence.find_by_id(command.id).await?;

        if task.user_id() != command.user_id {
            return Err(CompleteTaskError::Unauthorized);
        }

        task.complete()
            .map_err(|_| CompleteTaskError::UncompletedSubTasks)?;

        self.task_persistence.update_task(task).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository_traits::task_persistence::MockTaskPersistence;
    use chrono::Utc;
    use domain::entities::tasks::task::Task;

    #[tokio::test]
    async fn test_complete_task_success() {
        let mut mock_persistence = MockTaskPersistence::new();
        let task_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let command = CompleteTaskCommand {
            id: task_id,
            user_id,
        };

        let task = Task::new(user_id, "Test Task".to_string(), None, None);

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
        let user_id = Uuid::new_v4();
        let command = CompleteTaskCommand {
            id: task_id,
            user_id,
        };

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
        let command = CompleteTaskCommand {
            id: task_id,
            user_id,
        };

        let task = Task::new(user_id, "Test Task".to_string(), None, None);

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
        let command = CompleteTaskCommand {
            id: task_id,
            user_id,
        };

        let old_completed_at = Utc::now() - chrono::Duration::hours(1);
        let mut task = Task::new(user_id, "Already Completed Task".to_string(), None, None);
        task.complete().unwrap();

        mock_persistence
            .expect_find_by_id()
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
        let command = CompleteTaskCommand {
            id: task_id,
            user_id,
        };

        let task = Task::new(
            user_id,
            "Preserved Name".to_string(),
            None,
            Some("Preserved Description".to_string()),
        );

        mock_persistence
            .expect_find_by_id()
            .times(1)
            .returning(move |_| Ok(task.clone()));

        mock_persistence
            .expect_update_task()
            .withf(move |t| {
                t.user_id() == user_id
                    && t.title() == "Preserved Name"
                    && t.description() == Some("Preserved Description")
            })
            .times(1)
            .returning(|t| Ok(t));

        let use_case = CompleteTaskUseCase::new(Arc::new(mock_persistence));
        let result = use_case.execute(command).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_complete_task_unauthorized() {
        let mut mock_persistence = MockTaskPersistence::new();
        let task_id = Uuid::new_v4();
        let owner_id = Uuid::new_v4();
        let intruder_id = Uuid::new_v4();
        let command = CompleteTaskCommand {
            id: task_id,
            user_id: intruder_id,
        };

        let task = Task::new(owner_id, "Private Task".to_string(), None, None);

        mock_persistence
            .expect_find_by_id()
            .with(mockall::predicate::eq(task_id))
            .times(1)
            .returning(move |_| Ok(task.clone()));

        let use_case = CompleteTaskUseCase::new(Arc::new(mock_persistence));
        let result = use_case.execute(command).await;

        assert_eq!(result, Err(CompleteTaskError::Unauthorized));
    }
}
