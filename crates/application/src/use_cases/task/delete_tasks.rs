use crate::app_error::AppResult;
use domain::traits::task_persistence::TaskPersistence;
use std::sync::Arc;
use uuid::Uuid;

pub struct DeleteTasksUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl DeleteTasksUseCase {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    pub async fn execute(&self, task_ids: Vec<Uuid>) -> AppResult<Vec<Uuid>> {
        let mut deleted_ids = Vec::new();
        for task_id in task_ids {
            self.task_persistence.delete_task(task_id).await?;
            deleted_ids.push(task_id);
        }
        Ok(deleted_ids)
    }
}
