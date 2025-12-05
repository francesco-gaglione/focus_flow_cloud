use crate::app_error::AppResult;
use domain::entities::task::Task;
use domain::traits::task_persistence::TaskPersistence;
use std::sync::Arc;

pub struct GetTasksUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl GetTasksUseCase {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    pub async fn execute(&self) -> AppResult<Vec<Task>> {
        Ok(self.task_persistence.find_all().await?)
    }
}
