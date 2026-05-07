use crate::repository_traits::task_persistence::TaskPersistence;
use crate::{
    repository_traits::persistence_error::PersistenceError,
    use_cases::task::common::task_schedule_app_dto::TaskScheduleAppDto,
};
use chrono::{DateTime, Duration, NaiveDate, Utc};
use domain::entities::tasks::{
    subtask::Subtask, task::Task, task_priority::TaskPriority, task_schedule::TaskSchedule,
};
use std::sync::Arc;
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug, Error, PartialEq)]
pub enum CreateTaskError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type CreateTaskResult<T> = Result<T, CreateTaskError>;

#[derive(Debug, Clone)]
pub struct CreateTaskCommand {
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub category_id: Option<Uuid>,
    pub schedule: Option<TaskScheduleAppDto>,
    pub subtasks: Option<Vec<CreateSubtaskCommand>>,
    pub priority: Option<TaskPriority>,
}

#[derive(Debug, Clone)]
pub struct CreateSubtaskCommand {
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
}

pub struct CreateTaskUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl CreateTaskUseCase {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, command: CreateTaskCommand) -> CreateTaskResult<Uuid> {
        let mut task = Task::new(
            command.user_id,
            command.title,
            command.schedule.unwrap_or_default().into(),
            command.description,
        );

        if command.category_id.is_some() {
            task.update_category_id(command.category_id.unwrap());
        }

        if let Some(priority) = command.priority {
            task.set_priority(priority);
        }

        if command.subtasks.is_some() {
            for (index, s) in command.subtasks.unwrap().iter().enumerate() {
                task.add_subtask(Subtask::new(
                    s.title.clone(),
                    index as i16,
                    s.description.clone(),
                    None,
                ));
            }
        }

        tracing::info!("Creating task: {:?}", task);
        let result = self.task_persistence.create_task(task).await?;
        tracing::info!("Task created successfully: {}", result);

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository_traits::task_persistence::MockTaskPersistence;

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
            title: "New Task".to_string(),
            description: None,
            schedule: None,
            subtasks: None,
            category_id: Some(Uuid::new_v4()),
            priority: None,
        };

        let result = use_case.execute(command).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_uuid);
    }

    #[tokio::test]
    async fn test_create_task_with_due_date() {
        let mut mock_persistence = MockTaskPersistence::new();
        let expected_uuid = Uuid::new_v4();

        mock_persistence
            .expect_create_task()
            .returning(move |_| Ok(expected_uuid));

        let use_case = CreateTaskUseCase::new(Arc::new(mock_persistence));
        let command = CreateTaskCommand {
            user_id: Uuid::new_v4(),
            title: "New Task".to_string(),
            description: None,
            schedule: Some(TaskScheduleAppDto::AllDay {
                date: NaiveDate::from_ymd(2025, 1, 1),
            }),
            category_id: Some(Uuid::new_v4()),
            subtasks: None,
            priority: None,
        };

        let result = use_case.execute(command).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_uuid);
    }

    #[tokio::test]
    async fn test_create_task_persistence_error() {
        let mut mock_persistence = MockTaskPersistence::new();

        mock_persistence.expect_create_task().returning(move |_| {
            Err(PersistenceError::Unexpected(
                "Persistence error".to_string(),
            ))
        });

        let use_case = CreateTaskUseCase::new(Arc::new(mock_persistence));
        let command = CreateTaskCommand {
            user_id: Uuid::new_v4(),
            title: "New Task".to_string(),
            description: None,
            category_id: Some(Uuid::new_v4()),
            schedule: None,
            subtasks: None,
            priority: None,
        };

        let result = use_case.execute(command).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CreateTaskError::PersistenceError(_)
        ));
    }
}
