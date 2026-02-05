use domain::{
    error::persistence_error::PersistenceError, traits::task_persistence::TaskPersistence,
};
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error, PartialEq)]
pub enum DeleteTasksError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type DeleteTasksResult<T> = Result<T, DeleteTasksError>;

pub struct DeleteTasksUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl DeleteTasksUseCase {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    pub async fn execute(&self, task_ids: Vec<Uuid>) -> DeleteTasksResult<Vec<Uuid>> {
        let mut deleted_ids = Vec::new();
        for task_id in task_ids {
            self.task_persistence.delete_task(task_id).await?;
            deleted_ids.push(task_id);
        }
        Ok(deleted_ids)
    }
}

//TODO implement tests
