use crate::application::app_error::AppResult;
use crate::application::use_cases::persistance_command::create_task_data::CreateTaskData;
use crate::application::use_cases::persistance_command::update_task_data::UpdateTaskData;
use crate::domain::entities::task::Task;
use async_trait::async_trait;
use uuid::Uuid;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait TaskPersistence: Send + Sync {
    async fn create_task(&self, task: CreateTaskData) -> AppResult<Uuid>;
    async fn find_all(&self) -> AppResult<Vec<Task>>;
    async fn find_orphan_tasks(&self) -> AppResult<Vec<Task>>;
    async fn find_by_category_id(&self, category_id: Uuid) -> AppResult<Vec<Task>>;
    async fn find_by_id(&self, task_id: Uuid) -> AppResult<Task>;
    async fn update_task(&self, task_id: Uuid, task: UpdateTaskData) -> AppResult<Task>;
    async fn delete_task(&self, task: Uuid) -> AppResult<()>;
}
