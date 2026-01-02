use crate::app_error::AppResult;
use crate::use_cases::task::command::create_task::CreateTaskCommand;

use domain::entities::task::Task;
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

    pub async fn execute(&self, command: CreateTaskCommand) -> AppResult<Uuid> {
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
