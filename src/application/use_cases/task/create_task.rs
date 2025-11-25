use crate::application::app_error::AppResult;
use crate::application::traits::task_persistence::TaskPersistence;
use crate::application::use_cases::persistance_command::create_task_data::CreateTaskData;
use crate::application::use_cases::task::command::create_task::CreateTaskCommand;
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
            .create_task(CreateTaskData {
                name: task.name,
                description: task.description,
                category_id: task.category_id,
                scheduled_date: task.scheduled_date,
            })
            .await
    }
}
