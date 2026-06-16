use std::sync::Arc;

use thiserror::Error;
use tracing::{error, info, instrument};
use uuid::Uuid;

use crate::repository_traits::{
    persistence_error::PersistenceError, task_persistence::TaskPersistence,
};

#[derive(Debug, Error, PartialEq)]
pub enum DeleteSubTaskError {
    #[error("Task not found: {0}")]
    TaskNotFound(Uuid),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type DeleteSubTaskResult<T> = Result<T, DeleteSubTaskError>;

#[derive(Debug)]
pub struct DeleteSubTaskCommand {
    pub task_id: Uuid,
    pub subtask_id: Uuid,
    pub user_id: Uuid,
}

pub struct DeleteSubTaskUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl DeleteSubTaskUseCase {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, command: DeleteSubTaskCommand) -> DeleteSubTaskResult<()> {
        info!("Finding task: {:?}", command.task_id);
        let mut task = self
            .task_persistence
            .find_by_id(command.task_id)
            .await
            .map_err(|e| match e {
                PersistenceError::NotFound(_) => DeleteSubTaskError::TaskNotFound(command.task_id),
                other => DeleteSubTaskError::PersistenceError(other),
            })?;

        if task.user_id() != command.user_id {
            error!(
                "Unauthorized attempt to delete subtask from task: {:?} by user: {:?}",
                command.task_id, command.user_id
            );
            return Err(DeleteSubTaskError::Unauthorized);
        }

        info!("Removing subtask: {:?}", command.subtask_id);
        task.sub_tasks_mut().retain(|s| s.id() != command.subtask_id);

        self.task_persistence.update_task(task).await?;
        info!("Subtask deleted successfully");

        Ok(())
    }
}
