use std::sync::Arc;

use chrono::{DateTime, Utc};
use domain::entities::task::Task;
use thiserror::Error;
use uuid::Uuid;

use crate::persistence_traits::{
    persistence_error::PersistenceError, task_persistence::TaskPersistence,
};

#[derive(Debug, Clone, Error)]
pub enum GetScheduledTasksError {
    #[error("Task persistence error: {0}")]
    TaskPersistenceError(#[from] PersistenceError),
}

pub type GestScheduledTasksResult<T> = Result<T, GetScheduledTasksError>;

#[derive(Debug, Clone)]
pub struct GetScheduledTasksUseCaseCommand {
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
    pub completed: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct ScheduledTaskListOutput {
    pub scheduled_tasks: Vec<ScheduledTaskOutput>,
}

#[derive(Debug, Clone)]
pub struct ScheduledTaskOutput {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub scheduled_date: Option<DateTime<Utc>>,
    pub scheduled_end_date: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl From<&Task> for ScheduledTaskOutput {
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

pub struct GetScheduledTasksUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl GetScheduledTasksUseCase {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    pub async fn execute(
        &self,
        command: GetScheduledTasksUseCaseCommand,
    ) -> GestScheduledTasksResult<ScheduledTaskListOutput> {
        let res = self
            .task_persistence
            .find_scheduled_tasks(command.from, command.to, command.completed)
            .await?;
        Ok(ScheduledTaskListOutput {
            scheduled_tasks: res.iter().map(|t| t.into()).collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence_traits::{
        persistence_error::PersistenceError, task_persistence::MockTaskPersistence,
    };
    use chrono::Duration;

    fn make_task_minimal() -> Task {
        Task::reconstitute(
            Uuid::new_v4(),
            Uuid::new_v4(),
            None,
            "Minimal task".to_string(),
            None,
            None,
            None,
            None,
        )
    }

    fn make_task_full() -> Task {
        let now = Utc::now();
        Task::reconstitute(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Some(Uuid::new_v4()),
            "Full task".to_string(),
            Some("A description".to_string()),
            Some(now),
            Some(now + Duration::hours(1)),
            Some(now - Duration::hours(1)),
        )
    }

    #[test]
    fn test_from_task_maps_all_fields() {
        let task = make_task_full();
        let output = ScheduledTaskOutput::from(&task);
        assert_eq!(output.id, task.id());
        assert_eq!(output.user_id, task.user_id());
        assert_eq!(output.category_id, task.category_id());
        assert_eq!(output.name, task.name().to_string());
        assert_eq!(
            output.description,
            task.description().map(|d| d.to_string())
        );
        assert_eq!(output.scheduled_date, task.scheduled_date());
        assert_eq!(output.scheduled_end_date, task.scheduled_end_date());
        assert_eq!(output.completed_at, task.completed_at());
    }

    #[test]
    fn test_from_task_maps_optional_fields_as_none() {
        let task = make_task_minimal();
        let output = ScheduledTaskOutput::from(&task);
        assert_eq!(output.id, task.id());
        assert_eq!(output.user_id, task.user_id());
        assert!(output.category_id.is_none());
        assert_eq!(output.name, task.name().to_string());
        assert!(output.description.is_none());
        assert!(output.scheduled_date.is_none());
        assert!(output.scheduled_end_date.is_none());
        assert!(output.completed_at.is_none());
    }

    #[test]
    fn test_error_display() {
        let err = GetScheduledTasksError::TaskPersistenceError(PersistenceError::Unexpected(
            "db down".to_string(),
        ));
        assert!(err.to_string().contains("Task persistence error"));
        assert!(err.to_string().contains("db down"));
    }

    #[test]
    fn test_error_from_persistence_error() {
        let persistence_err = PersistenceError::NotFound("x".to_string());
        let err = GetScheduledTasksError::from(persistence_err.clone());
        assert!(matches!(
            err,
            GetScheduledTasksError::TaskPersistenceError(_)
        ));
    }

    #[tokio::test]
    async fn test_execute_returns_empty_list() {
        let mut mock = MockTaskPersistence::new();
        mock.expect_find_scheduled_tasks()
            .returning(|_, _, _| Ok(vec![]));

        let use_case = GetScheduledTasksUseCase::new(Arc::new(mock));
        let result = use_case
            .execute(GetScheduledTasksUseCaseCommand {
                from: None,
                to: None,
                completed: None,
            })
            .await
            .unwrap();

        assert!(result.scheduled_tasks.is_empty());
    }

    #[tokio::test]
    async fn test_execute_maps_single_task() {
        let task = make_task_full();
        let task_clone = task.clone();
        let mut mock = MockTaskPersistence::new();
        mock.expect_find_scheduled_tasks()
            .returning(move |_, _, _| Ok(vec![task_clone.clone()]));

        let use_case = GetScheduledTasksUseCase::new(Arc::new(mock));
        let result = use_case
            .execute(GetScheduledTasksUseCaseCommand {
                from: None,
                to: None,
                completed: None,
            })
            .await
            .unwrap();

        assert_eq!(result.scheduled_tasks.len(), 1);
        let out = &result.scheduled_tasks[0];
        assert_eq!(out.id, task.id());
        assert_eq!(out.user_id, task.user_id());
        assert_eq!(out.category_id, task.category_id());
        assert_eq!(out.name, task.name().to_string());
        assert_eq!(out.description, task.description().map(|d| d.to_string()));
        assert_eq!(out.scheduled_date, task.scheduled_date());
        assert_eq!(out.scheduled_end_date, task.scheduled_end_date());
        assert_eq!(out.completed_at, task.completed_at());
    }

    #[tokio::test]
    async fn test_execute_maps_multiple_tasks() {
        let tasks = vec![make_task_minimal(), make_task_full()];
        let tasks_clone = tasks.clone();
        let mut mock = MockTaskPersistence::new();
        mock.expect_find_scheduled_tasks()
            .returning(move |_, _, _| Ok(tasks_clone.clone()));

        let use_case = GetScheduledTasksUseCase::new(Arc::new(mock));
        let result = use_case
            .execute(GetScheduledTasksUseCaseCommand {
                from: None,
                to: None,
                completed: None,
            })
            .await
            .unwrap();

        assert_eq!(result.scheduled_tasks.len(), 2);
    }

    #[tokio::test]
    async fn test_execute_propagates_persistence_error() {
        let mut mock = MockTaskPersistence::new();
        mock.expect_find_scheduled_tasks()
            .returning(|_, _, _| Err(PersistenceError::Unexpected("db error".to_string())));

        let use_case = GetScheduledTasksUseCase::new(Arc::new(mock));
        let result = use_case
            .execute(GetScheduledTasksUseCaseCommand {
                from: None,
                to: None,
                completed: None,
            })
            .await;

        assert!(matches!(
            result.err().unwrap(),
            GetScheduledTasksError::TaskPersistenceError(_)
        ));
    }

    #[tokio::test]
    async fn test_execute_passes_from_filter() {
        let from_dt = Utc::now();
        let mut mock = MockTaskPersistence::new();
        mock.expect_find_scheduled_tasks()
            .withf(move |from, to, completed| {
                *from == Some(from_dt) && to.is_none() && completed.is_none()
            })
            .returning(|_, _, _| Ok(vec![]));

        let use_case = GetScheduledTasksUseCase::new(Arc::new(mock));
        let result = use_case
            .execute(GetScheduledTasksUseCaseCommand {
                from: Some(from_dt),
                to: None,
                completed: None,
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_execute_passes_to_filter() {
        let to_dt = Utc::now();
        let mut mock = MockTaskPersistence::new();
        mock.expect_find_scheduled_tasks()
            .withf(move |from, to, completed| {
                from.is_none() && *to == Some(to_dt) && completed.is_none()
            })
            .returning(|_, _, _| Ok(vec![]));

        let use_case = GetScheduledTasksUseCase::new(Arc::new(mock));
        let result = use_case
            .execute(GetScheduledTasksUseCaseCommand {
                from: None,
                to: Some(to_dt),
                completed: None,
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_execute_passes_completed_some_true() {
        let mut mock = MockTaskPersistence::new();
        mock.expect_find_scheduled_tasks()
            .withf(|_, _, completed| *completed == Some(true))
            .returning(|_, _, _| Ok(vec![]));

        let use_case = GetScheduledTasksUseCase::new(Arc::new(mock));
        let result = use_case
            .execute(GetScheduledTasksUseCaseCommand {
                from: None,
                to: None,
                completed: Some(true),
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_execute_passes_completed_some_false() {
        let mut mock = MockTaskPersistence::new();
        mock.expect_find_scheduled_tasks()
            .withf(|_, _, completed| *completed == Some(false))
            .returning(|_, _, _| Ok(vec![]));

        let use_case = GetScheduledTasksUseCase::new(Arc::new(mock));
        let result = use_case
            .execute(GetScheduledTasksUseCaseCommand {
                from: None,
                to: None,
                completed: Some(false),
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_execute_passes_all_filters() {
        let from_dt = Utc::now();
        let to_dt = from_dt + Duration::hours(24);
        let mut mock = MockTaskPersistence::new();
        mock.expect_find_scheduled_tasks()
            .withf(move |from, to, completed| {
                *from == Some(from_dt) && *to == Some(to_dt) && *completed == Some(false)
            })
            .returning(|_, _, _| Ok(vec![]));

        let use_case = GetScheduledTasksUseCase::new(Arc::new(mock));
        let result = use_case
            .execute(GetScheduledTasksUseCaseCommand {
                from: Some(from_dt),
                to: Some(to_dt),
                completed: Some(false),
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_execute_passes_none_filters() {
        let mut mock = MockTaskPersistence::new();
        mock.expect_find_scheduled_tasks()
            .withf(|from, to, completed| from.is_none() && to.is_none() && completed.is_none())
            .returning(|_, _, _| Ok(vec![]));

        let use_case = GetScheduledTasksUseCase::new(Arc::new(mock));
        let result = use_case
            .execute(GetScheduledTasksUseCaseCommand {
                from: None,
                to: None,
                completed: None,
            })
            .await;

        assert!(result.is_ok());
    }
}
