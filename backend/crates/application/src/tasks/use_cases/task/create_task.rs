use crate::shared::traits::reminder_persistence::ReminderPersistence;
use crate::shared::traits::reminder_worker_port::{ReminderWorkerPort, WorkerPortError};
use crate::tasks::traits::task_persistence::TaskPersistence;
use crate::{
    shared::traits::persistence_error::PersistenceError,
    tasks::use_cases::task::common::task_schedule_app_dto::TaskScheduleAppDto,
};
use chrono::{DateTime, Utc};
use domain::tasks::entities::reminder::Reminder;
use domain::tasks::entities::{subtask::Subtask, task::Task, task_priority::TaskPriority};
use std::sync::Arc;
use thiserror::Error;
use tracing::{info, instrument};
use uuid::Uuid;

#[derive(Debug, Error, PartialEq)]
pub enum CreateTaskError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),

    #[error("Worker port error: {0}")]
    WorkerPortError(#[from] WorkerPortError),
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
    pub reminders: Option<Vec<AddReminderCommand>>,
}

#[derive(Debug, Clone)]
pub struct CreateSubtaskCommand {
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AddReminderCommand {
    pub date: DateTime<Utc>,
}

pub struct CreateTaskUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
    reminder_persistence: Arc<dyn ReminderPersistence>,
    reminder_worker: Arc<dyn ReminderWorkerPort>,
}

impl CreateTaskUseCase {
    pub fn new(
        task_persistence: Arc<dyn TaskPersistence>,
        reminder_persistence: Arc<dyn ReminderPersistence>,
        reminder_worker: Arc<dyn ReminderWorkerPort>,
    ) -> Self {
        Self {
            task_persistence,
            reminder_persistence,
            reminder_worker,
        }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, command: CreateTaskCommand) -> CreateTaskResult<Uuid> {
        let mut task = Task::new(
            command.user_id,
            command.title.clone(),
            command.schedule.unwrap_or_default().into(),
            command.description.clone(),
        );
        for reminder in command.reminders.unwrap_or_default() {
            task.add_reminder(Reminder::new(
                Some(task.id()),
                command.user_id,
                "".to_string(), //FIXME
                reminder.date,
                "".to_string(), //FIXME
            ));
        }

        if let Some(cat_id) = command.category_id {
            task.update_category_id(cat_id);
        }

        task.set_priority(command.priority.unwrap_or(TaskPriority::Low));

        if let Some(subtasks) = command.subtasks {
            for (index, s) in subtasks.iter().enumerate() {
                task.add_subtask(Subtask::new(
                    s.title.clone(),
                    index as i16,
                    s.description.clone(),
                    None,
                ));
            }
        }

        info!("Creating task: {:?}", task);
        let task_id = self.task_persistence.create_task(task.clone()).await?;
        info!("Task created successfully: {}", task_id);

        for reminder in task.reminders() {
            let reminder_entity = Reminder::new(
                Some(task_id),
                command.user_id,
                command.title.clone(),
                reminder.date(),
                command.description.clone().unwrap_or_default(),
            );
            info!("Scheduling reminder {:?}", reminder_entity);
            let reminder_id = self
                .reminder_persistence
                .save_reminder(reminder_entity)
                .await?;
            info!("Reminder saved: {}", reminder_id);
            self.reminder_worker
                .schedule(reminder_id, reminder.date())
                .await?;
            info!("Reminder scheduled: {}", reminder_id);
        }

        Ok(task_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::traits::reminder_persistence::MockReminderPersistence;
    use crate::shared::traits::reminder_worker_port::MockReminderWorkerPort;
    use crate::tasks::traits::task_persistence::MockTaskPersistence;
    use chrono::NaiveDate;

    fn make_use_case(
        mock_persistence: MockTaskPersistence,
        mock_reminder_persistence: MockReminderPersistence,
        mock_worker: MockReminderWorkerPort,
    ) -> CreateTaskUseCase {
        CreateTaskUseCase::new(
            Arc::new(mock_persistence),
            Arc::new(mock_reminder_persistence),
            Arc::new(mock_worker),
        )
    }

    #[tokio::test]
    async fn test_create_task_success() {
        let mut mock_persistence = MockTaskPersistence::new();
        let expected_uuid = Uuid::new_v4();

        mock_persistence
            .expect_create_task()
            .returning(move |_| Ok(expected_uuid));

        let result = make_use_case(
            mock_persistence,
            MockReminderPersistence::new(),
            MockReminderWorkerPort::new(),
        )
        .execute(CreateTaskCommand {
            user_id: Uuid::new_v4(),
            title: "New Task".to_string(),
            description: None,
            schedule: None,
            subtasks: None,
            category_id: Some(Uuid::new_v4()),
            priority: None,
            reminders: None,
        })
        .await;

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

        let result = make_use_case(
            mock_persistence,
            MockReminderPersistence::new(),
            MockReminderWorkerPort::new(),
        )
        .execute(CreateTaskCommand {
            user_id: Uuid::new_v4(),
            title: "New Task".to_string(),
            description: None,
            schedule: Some(TaskScheduleAppDto::AllDay {
                date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            }),
            category_id: Some(Uuid::new_v4()),
            subtasks: None,
            priority: None,
            reminders: None,
        })
        .await;

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

        let result = make_use_case(
            mock_persistence,
            MockReminderPersistence::new(),
            MockReminderWorkerPort::new(),
        )
        .execute(CreateTaskCommand {
            user_id: Uuid::new_v4(),
            title: "New Task".to_string(),
            description: None,
            category_id: Some(Uuid::new_v4()),
            schedule: None,
            subtasks: None,
            priority: None,
            reminders: None,
        })
        .await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CreateTaskError::PersistenceError(_)
        ));
    }
}
