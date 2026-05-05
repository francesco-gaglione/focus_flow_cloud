use std::sync::Arc;

use thiserror::Error;
use tracing::{error, info, instrument};
use uuid::Uuid;

use crate::repository_traits::{
    persistence_error::PersistenceError, task_persistence::TaskPersistence,
};

#[derive(Debug, Error, PartialEq)]
pub enum CompleteSubTaskError {
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

pub type CompleteSubTaskResult<T> = Result<T, CompleteSubTaskError>;

#[derive(Debug)]
pub struct CompleteSubTaskCommand {
    pub task_id: Uuid,
    pub sub_task_id: Uuid,
    pub user_id: Uuid,
}

pub struct CompleteSubTaskUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl CompleteSubTaskUseCase {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, command: CompleteSubTaskCommand) -> CompleteSubTaskResult<()> {
        info!("Finding task: {:?}", command.task_id);
        let mut task = self.task_persistence.find_by_id(command.task_id).await?;

        if task.user_id() != command.user_id {
            error!(
                "Unauthorized attempt to complete subtask: {:?} by user: {:?}",
                command.sub_task_id, command.user_id
            );
            return Err(CompleteSubTaskError::Unauthorized);
        }

        info!("Completing subtask: {:?}", command.sub_task_id);

        let sub_task = task
            .sub_tasks_mut()
            .iter_mut()
            .find(|s| s.id() == command.sub_task_id)
            .ok_or_else(|| {
                error!("Subtask not found: {:?}", command.sub_task_id);
                CompleteSubTaskError::SubTaskNotFound(command.sub_task_id)
            })?;
        sub_task.mark_completed();

        info!("Updating task: {:?}", command.task_id);
        self.task_persistence.update_task(task).await?;
        info!("Task completed successfully: {:?}", command.task_id);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository_traits::task_persistence::MockTaskPersistence;
    use chrono::Utc;
    use domain::entities::tasks::task::Task;
}
