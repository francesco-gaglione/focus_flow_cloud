use std::sync::Arc;

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
pub enum DeleteReminderError {
    #[error("Task not found: {0}")]
    TaskNotFound(Uuid),

    #[error("Reminder not found: {0}")]
    ReminderNotFound(Uuid),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),

    #[error("Worker port error: {0}")]
    WorkerPortError(#[from] WorkerPortError),
}

pub type DeleteReminderResult<T> = Result<T, DeleteReminderError>;

#[derive(Debug)]
pub struct DeleteReminderCommand {
    pub task_id: Uuid,
    pub reminder_id: Uuid,
    pub user_id: Uuid,
}

pub struct DeleteReminderUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
    reminder_persistence: Arc<dyn ReminderPersistence>,
    reminder_worker: Arc<dyn ReminderWorkerPort>,
}

impl DeleteReminderUseCase {
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
    pub async fn execute(&self, command: DeleteReminderCommand) -> DeleteReminderResult<()> {
        info!("Finding task: {:?}", command.task_id);
        let task = self
            .task_persistence
            .find_by_id(command.task_id)
            .await
            .map_err(|e| match e {
                PersistenceError::NotFound(_) => {
                    DeleteReminderError::TaskNotFound(command.task_id)
                }
                other => DeleteReminderError::PersistenceError(other),
            })?;

        if task.user_id() != command.user_id {
            error!(
                "Unauthorized attempt to delete reminder from task: {:?} by user: {:?}",
                command.task_id, command.user_id
            );
            return Err(DeleteReminderError::Unauthorized);
        }

        info!("Deleting reminder: {:?}", command.reminder_id);
        self.reminder_persistence
            .delete_reminder(command.reminder_id)
            .await?;

        info!("Unscheduling reminder: {:?}", command.reminder_id);
        // Best-effort unschedule - ignore worker errors (reminder may not be pending)
        let _ = self.reminder_worker.unschedule(command.reminder_id).await;

        info!("Reminder deleted successfully");
        Ok(())
    }
}
