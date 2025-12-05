use crate::app_error::AppResult;
use crate::traits::task_persistence::TaskPersistence;
use crate::use_cases::persistance_command::create_task_data::CreateTaskData;
use crate::use_cases::task::command::create_task::CreateTaskCommand;
use std::sync::Arc;
use uuid::Uuid;

pub struct CreateTaskUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl CreateTaskUseCase {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    pub async fn execute(&self, task: CreateTaskCommand) -> AppResult<Uuid> {
        self.task_persistence
            .create_task(CreateTaskData::new(
                task.name,
                task.description,
                task.category_id,
                task.scheduled_date,
            ))
            .await
    }
}
