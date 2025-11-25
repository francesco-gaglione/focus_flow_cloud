use crate::application::app_error::AppResult;
use crate::application::traits::task_persistence::TaskPersistence;
use crate::application::use_cases::commands::create_task::CreateTaskCommand;
use crate::application::use_cases::commands::update_task::UpdateTaskCommand;
use crate::application::use_cases::persistance_command::create_task_data::CreateTaskData;
use crate::application::use_cases::persistance_command::update_task_data::UpdateTaskData;
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

    pub async fn orphan_tasks(&self) -> AppResult<Vec<Task>> {
        self.task_persistence.find_orphan_tasks().await
    }

    pub async fn create_task(&self, task: CreateTaskCommand) -> AppResult<Uuid> {
        self.task_persistence
            .create_task(CreateTaskData {
                name: task.name,
                description: task.description,
                category_id: task.category_id,
                scheduled_date: task.scheduled_date,
            })
            .await
    }

    pub async fn delete_tasks(&self, task_ids: Vec<Uuid>) -> AppResult<Vec<Uuid>> {
        let mut deleted_ids = Vec::new();
        for task_id in task_ids {
            self.task_persistence.delete_task(task_id).await?;
            deleted_ids.push(task_id);
        }
        Ok(deleted_ids)
    }

    pub async fn update_task(&self, task: UpdateTaskCommand) -> AppResult<Task> {
        self.task_persistence
            .update_task(
                task.id,
                UpdateTaskData {
                    category_id: task.category_id,
                    name: task.name.clone(),
                    description: task.description.clone(),
                    scheduled_date: task.scheduled_date,
                    completed_at: task.completed_at,
                },
            )
            .await
    }
}
