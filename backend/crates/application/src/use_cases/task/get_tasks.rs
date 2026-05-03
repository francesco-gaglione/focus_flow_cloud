use crate::repository_traits::persistence_error::PersistenceError;
use crate::repository_traits::task_persistence::TaskPersistence;
use chrono::{DateTime, Utc};
use domain::entities::tasks::subtask::Subtask;
use domain::entities::tasks::task::Task;
use domain::entities::tasks::task_priority::TaskPriority;
use std::sync::Arc;
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug, Error, PartialEq)]
pub enum GetTaskError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type GetTasksResult<T> = Result<T, GetTaskError>;

#[derive(Debug)]
pub struct GetTasksCommand {
    pub completed: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct TaskOutput {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<TaskPriority>,
    pub due_date: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub subtasks: Vec<SubTaskOutput>,
}

#[derive(Debug, Clone)]
pub struct SubTaskOutput {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub is_completed: bool,
    pub sort_order: i16,
}

impl From<&Task> for TaskOutput {
    fn from(value: &Task) -> Self {
        Self {
            id: value.id(),
            user_id: value.user_id(),
            title: value.title().to_string(),
            description: value.description().map(|d| d.to_string()),
            priority: value.priority(),
            due_date: value.due_date(),
            completed_at: value.completed_at(),
            subtasks: value.sub_tasks().iter().map(|s| s.into()).collect(),
        }
    }
}

impl From<&Subtask> for SubTaskOutput {
    fn from(value: &Subtask) -> Self {
        Self {
            id: value.id(),
            title: value.title().to_string(),
            description: value.description().map(|d| d.to_string()),
            is_completed: value.is_completed(),
            sort_order: value.sort_order(),
        }
    }
}

pub struct GetTasksUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl GetTasksUseCase {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, command: GetTasksCommand) -> GetTasksResult<Vec<TaskOutput>> {
        tracing::info!("Fetching tasks with completed: {:?}", command.completed);
        let res = self
            .task_persistence
            .find_all(command.completed.unwrap_or(false))
            .await?;
        tracing::info!("Fetched {} tasks", res.len());
        Ok(res.iter().map(|t| t.into()).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository_traits::task_persistence::MockTaskPersistence;

    #[tokio::test]
    async fn test_get_tasks_success() {
        let mut mock_persistence = MockTaskPersistence::new();
        let expected_tasks = vec![Task::new(Uuid::new_v4(), "Task 1".to_string(), None, None)];
        let returned_tasks = expected_tasks.clone();

        mock_persistence
            .expect_find_all()
            .with(mockall::predicate::eq(false))
            .returning(move |_| Ok(returned_tasks.clone()));

        let use_case = GetTasksUseCase::new(Arc::new(mock_persistence));
        let command = GetTasksCommand { completed: None };
        let result = use_case.execute(command).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_get_tasks_completed() {
        let mut mock_persistence = MockTaskPersistence::new();

        mock_persistence
            .expect_find_all()
            .with(mockall::predicate::eq(true))
            .returning(move |_| Ok(vec![]));

        let use_case = GetTasksUseCase::new(Arc::new(mock_persistence));
        let command = GetTasksCommand {
            completed: Some(true),
        };
        let result = use_case.execute(command).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }
}
