use std::sync::Arc;

use domain::entities::tasks::subtask::Subtask;
use thiserror::Error;
use tracing::{error, info, instrument};
use uuid::Uuid;

use crate::repository_traits::{
    persistence_error::PersistenceError, task_persistence::TaskPersistence,
};

#[derive(Debug, Error, PartialEq)]
pub enum AddSubTaskError {
    #[error("Task not found: {0}")]
    TaskNotFound(Uuid),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type AddSubTaskResult<T> = Result<T, AddSubTaskError>;

#[derive(Debug)]
pub struct AddSubTaskCommand {
    pub task_id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
}

pub struct AddSubTaskUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl AddSubTaskUseCase {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, command: AddSubTaskCommand) -> AddSubTaskResult<Uuid> {
        info!("Finding task: {:?}", command.task_id);
        let mut task = self
            .task_persistence
            .find_by_id(command.task_id)
            .await
            .map_err(|e| match e {
                PersistenceError::NotFound(_) => AddSubTaskError::TaskNotFound(command.task_id),
                other => AddSubTaskError::PersistenceError(other),
            })?;

        if task.user_id() != command.user_id {
            error!(
                "Unauthorized attempt to add subtask to task: {:?} by user: {:?}",
                command.task_id, command.user_id
            );
            return Err(AddSubTaskError::Unauthorized);
        }

        let sort_order = task.sub_tasks().len() as i16;
        let subtask = Subtask::new(command.title, sort_order, command.description, None);
        let subtask_id = subtask.id();

        info!("Adding subtask to task: {:?}", command.task_id);
        task.add_subtask(subtask);

        self.task_persistence.update_task(task).await?;
        info!("Subtask added successfully, id: {:?}", subtask_id);

        Ok(subtask_id)
    }
}
