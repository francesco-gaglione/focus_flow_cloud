use crate::app_error::AppResult;
use crate::use_cases::task::command::update_task::UpdateTaskCommand;
use domain::entities::task::Task;
use domain::traits::task_persistence::TaskPersistence;
use std::sync::Arc;

pub struct UpdateTaskUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl UpdateTaskUseCase {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    pub async fn execute(&self, command: UpdateTaskCommand) -> AppResult<Task> {
        let mut task = self.task_persistence.find_by_id(command.id).await?;

        if let Some(name) = command.name {
            task.update_name(name);
        }
        if let Some(category_id) = command.category_id {
            task.update_category(Some(category_id));
        }
        if let Some(description) = command.description {
            task.update_description(Some(description));
        }
        if let Some(scheduled_date) = command.scheduled_date {
            task.update_scheduled_date(Some(scheduled_date));
        }

        if let Some(completed_at) = command.completed_at {
            task.update_completed_at(Some(completed_at));
        }

        Ok(self.task_persistence.update_task(task).await?)
    }
}
