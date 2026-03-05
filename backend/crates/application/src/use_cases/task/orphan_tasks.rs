use crate::persistence_traits::persistence_error::PersistenceError;
use crate::persistence_traits::task_persistence::TaskPersistence;
use chrono::{DateTime, Utc};
use domain::entities::task::Task;
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error, PartialEq)]
pub enum OrphanTasksError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type OrphanTasksResult<T> = Result<T, OrphanTasksError>;

pub struct GetOrphanTasksCommand {
    pub completed: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct TaskOutput {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub scheduled_date: Option<DateTime<Utc>>,
    pub scheduled_end_date: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl From<&Task> for TaskOutput {
    fn from(value: &Task) -> Self {
        Self {
            id: value.id(),
            user_id: value.user_id(),
            category_id: value.category_id(),
            name: value.name().to_string(),
            description: value.description().map(|d| d.to_string()),
            scheduled_date: value.scheduled_date(),
            scheduled_end_date: value.scheduled_end_date(),
            completed_at: value.completed_at(),
        }
    }
}

pub struct OrphanTasksUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl OrphanTasksUseCase {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    pub async fn execute(
        &self,
        command: GetOrphanTasksCommand,
    ) -> OrphanTasksResult<Vec<TaskOutput>> {
        let res = self
            .task_persistence
            .find_orphan_tasks(command.completed.unwrap_or(false))
            .await?;
        Ok(res.iter().map(|t| t.into()).collect())
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
    }

    #[tokio::test]
    async fn test_orphan_tasks_completed() {
        let mut mock_persistence = MockTaskPersistence::new();

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
    }
}
