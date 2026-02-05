use crate::use_cases::task::command::create_task::CreateTaskCommand;

use domain::entities::task::Task;
use domain::error::persistence_error::PersistenceError;
use domain::traits::task_persistence::TaskPersistence;
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error, PartialEq)]
pub enum CreateTaskError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type CreateTaskResult<T> = Result<T, CreateTaskError>;

pub struct CreateTaskUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl CreateTaskUseCase {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    pub async fn execute(&self, command: CreateTaskCommand) -> CreateTaskResult<Uuid> {
        let task = Task::create(
            command.user_id,
            command.category_id,
            command.name.clone(),
            command.description.clone(),
            command.scheduled_date,
        );

        let result = self.task_persistence.create_task(task).await;

        Ok(result?)
    }
}
