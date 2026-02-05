use domain::entities::task::Task;
use domain::error::persistence_error::PersistenceError;
use domain::traits::task_persistence::TaskPersistence;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum GetTaskError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type GetTasksResult<T> = Result<T, GetTaskError>;

pub struct GetTasksUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl GetTasksUseCase {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    pub async fn execute(&self) -> GetTasksResult<Vec<Task>> {
        Ok(self.task_persistence.find_all().await?)
    }
}

//TODO unit tests
