use crate::app_error::AppResult;
use crate::use_cases::task::command::create_task::CreateTaskCommand;
use domain::traits::task_persistence::TaskPersistence;
use std::sync::Arc;
use uuid::Uuid;

pub struct CreateTaskUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl CreateTaskUseCase {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    pub async fn execute(&self, task_cmd: CreateTaskCommand) -> AppResult<Uuid> {
        let task = domain::entities::task::Task::create(
            task_cmd.category_id,
            task_cmd.name,
            task_cmd.description,
            task_cmd.scheduled_date,
        );

        let result = self.task_persistence.create_task(task).await;

        Ok(result?)
    }
}
