use crate::application::app_error::AppResult;
use crate::application::traits::task_persistence::TaskPersistence;
use crate::application::use_cases::persistance_command::update_task_data::UpdateTaskData;
use crate::application::use_cases::task::command::update_task::UpdateTaskCommand;
use crate::domain::entities::task::Task;
use std::sync::Arc;

pub struct UpdateTaskUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl UpdateTaskUseCase {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    pub async fn execute(&self, task: UpdateTaskCommand) -> AppResult<Task> {
        self.task_persistence
            .update_task(
                task.id,
                UpdateTaskData::new(
                    task.category_id,
                    task.name,
                    task.description,
                    task.scheduled_date,
                    task.completed_at,
                ),
            )
            .await
    }
}
