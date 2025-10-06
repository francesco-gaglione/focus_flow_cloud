use crate::application::app_error::AppResult;
use crate::application::traits::TaskPersistence;
use crate::application::use_cases::commands::create_task::CreateTaskCommand;
use crate::application::use_cases::commands::update_task::UpdateTaskCommand;
use crate::domain::entities::task::Task;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct TaskUseCases {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl TaskUseCases {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    pub async fn create_task(&self, task: &CreateTaskCommand) -> AppResult<Uuid> {
        self.task_persistence.create_task(task).await
    }

    pub async fn delete_tasks(&self, task_ids: Vec<Uuid>) -> AppResult<Vec<Uuid>> {
        let mut deleted_ids = Vec::new();
        for task_id in task_ids {
            self.task_persistence.delete_task(task_id).await?;
            deleted_ids.push(task_id);
        }
        Ok(deleted_ids)
    }

    pub async fn update_task(&self, task: &UpdateTaskCommand) -> AppResult<Task> {
        self.task_persistence.update_task(task).await
    }
}
