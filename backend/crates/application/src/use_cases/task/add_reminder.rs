use std::sync::Arc;

use chrono::{DateTime, Utc};
use domain::entities::reminder::Reminder;
use thiserror::Error;
use tracing::{error, info, instrument};
use uuid::Uuid;

use crate::repository_traits::{
    persistence_error::PersistenceError,
    reminder_persistence::ReminderPersistence,
    reminder_worker_port::{ReminderWorkerPort, WorkerPortError},
    task_persistence::TaskPersistence,
};

#[derive(Debug, Error, PartialEq)]
pub enum AddReminderToTaskError {
    #[error("Task not found: {0}")]
    TaskNotFound(Uuid),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),

    #[error("Worker port error: {0}")]
    WorkerPortError(#[from] WorkerPortError),
}

pub type AddReminderToTaskResult<T> = Result<T, AddReminderToTaskError>;

#[derive(Debug)]
pub struct AddReminderToTaskCommand {
    pub task_id: Uuid,
    pub user_id: Uuid,
    pub date: DateTime<Utc>,
}

pub struct AddReminderToTaskUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
    reminder_persistence: Arc<dyn ReminderPersistence>,
    reminder_worker: Arc<dyn ReminderWorkerPort>,
}

impl AddReminderToTaskUseCase {
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
    pub async fn execute(&self, command: AddReminderToTaskCommand) -> AddReminderToTaskResult<Uuid> {
        info!("Finding task: {:?}", command.task_id);
        let task = self
            .task_persistence
            .find_by_id(command.task_id)
            .await
            .map_err(|e| match e {
                PersistenceError::NotFound(_) => {
                    AddReminderToTaskError::TaskNotFound(command.task_id)
                }
                other => AddReminderToTaskError::PersistenceError(other),
            })?;

        if task.user_id() != command.user_id {
            error!(
                "Unauthorized attempt to add reminder to task: {:?} by user: {:?}",
                command.task_id, command.user_id
            );
            return Err(AddReminderToTaskError::Unauthorized);
        }

        let reminder = Reminder::new(
            Some(command.task_id),
            command.user_id,
            task.title().to_string(),
            command.date,
            task.description().unwrap_or("").to_string(),
        );
        let reminder_id = reminder.id();

        info!("Saving reminder: {:?}", reminder_id);
        self.reminder_persistence.save_reminder(reminder).await?;

        info!("Scheduling reminder: {:?}", reminder_id);
        self.reminder_worker
            .schedule(reminder_id, command.date)
            .await?;

        info!("Reminder added successfully: {:?}", reminder_id);
        Ok(reminder_id)
    }
}
