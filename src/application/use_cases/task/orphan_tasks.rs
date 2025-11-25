use crate::application::app_error::AppResult;
use crate::application::traits::task_persistence::TaskPersistence;
use crate::domain::entities::task::Task;
use std::sync::Arc;

pub struct OrphanTasksUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl OrphanTasksUseCase {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    pub async fn execute(&self) -> AppResult<Vec<Task>> {
        self.task_persistence.find_orphan_tasks().await
    }
}
