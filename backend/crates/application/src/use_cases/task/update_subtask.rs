use std::sync::Arc;

use thiserror::Error;
use tracing::{error, info, instrument};
use uuid::Uuid;

use crate::repository_traits::{
    persistence_error::PersistenceError, task_persistence::TaskPersistence,
};

#[derive(Debug, Error, PartialEq)]
pub enum UpdateSubTaskError {
    #[error("Task not found: {0}")]
    TaskNotFound(Uuid),

    #[error("SubTask not found: {0}")]
    SubTaskNotFound(Uuid),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Sub-tasks must be completed before marking task as completed")]
    UncompletedSubTasks,

    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type UpdateSubTaskResult<T> = Result<T, UpdateSubTaskError>;

#[derive(Debug)]
pub struct UpdateSubTaskCommand {
    pub task_id: Uuid,
    pub sub_task_id: Uuid,
    pub user_id: Uuid,
    pub completed: Option<bool>,
}

pub struct UpdateSubTaskUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl UpdateSubTaskUseCase {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, command: UpdateSubTaskCommand) -> UpdateSubTaskResult<()> {
        info!("Finding task: {:?}", command.task_id);
        let mut task = self.task_persistence.find_by_id(command.task_id).await?;

        if task.user_id() != command.user_id {
            error!(
                "Unauthorized attempt to complete subtask: {:?} by user: {:?}",
                command.sub_task_id, command.user_id
            );
            return Err(UpdateSubTaskError::Unauthorized);
        }

        if let Some(completed) = command.completed {
            let sub_task = task
                .sub_tasks_mut()
                .iter_mut()
                .find(|s| s.id() == command.sub_task_id)
                .ok_or_else(|| {
                    error!("Subtask not found: {:?}", command.sub_task_id);
                    UpdateSubTaskError::SubTaskNotFound(command.sub_task_id)
                })?;
            if completed {
                info!("Completing subtask: {:?}", command.sub_task_id);
                sub_task.mark_completed();
            } else {
                sub_task.mark_incomplete();
            }
        }

        info!("Updating subtask: {:?}", command.task_id);
        self.task_persistence.update_task(task).await?;
        info!("Subtask updated successfully: {:?}", command.task_id);

        Ok(())
    }
}
