use domain::entities::task::Task;
use domain::error::persistence_error::PersistenceError;
use domain::traits::task_persistence::TaskPersistence;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum OrphanTasksError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type OrphanTasksResult<T> = Result<T, OrphanTasksError>;

pub struct OrphanTasksUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl OrphanTasksUseCase {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    pub async fn execute(&self) -> OrphanTasksResult<Vec<Task>> {
        Ok(self.task_persistence.find_orphan_tasks().await?)
    }
}

//TODO unit tests
