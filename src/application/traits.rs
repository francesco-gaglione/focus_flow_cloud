use crate::application::app_error::AppResult;
use crate::application::use_cases::commands::create_category::CreateCategoryCommand;
use crate::application::use_cases::commands::create_task::CreateTaskCommand;
use crate::application::use_cases::commands::update_category::UpdateCategoryCommand;
use crate::application::use_cases::commands::update_task::UpdateTaskCommand;
use crate::application::use_cases::persistance_command::create_manual_session_data::CreateManualSessionData;
use crate::domain::entities::category::Category;
use crate::domain::entities::focus_session::FocusSession;
use crate::domain::entities::task::Task;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait CategoryPersistence: Send + Sync {
    async fn create_category(&self, category: &CreateCategoryCommand) -> AppResult<()>;
    async fn find_all(&self) -> AppResult<Vec<Category>>;
    async fn update_category(&self, category: &UpdateCategoryCommand) -> AppResult<Category>;
    async fn delete_category_by_id(&self, category_id: Uuid) -> AppResult<()>;
}

#[async_trait]
pub trait TaskPersistence: Send + Sync {
    async fn create_task(&self, task: &CreateTaskCommand) -> AppResult<Uuid>;
    async fn find_all(&self) -> AppResult<Vec<Task>>;
    async fn find_orphan_tasks(&self) -> AppResult<Vec<Task>>;
    async fn find_by_category_id(&self, category_id: Uuid) -> AppResult<Vec<Task>>;
    async fn update_task(&self, task: &UpdateTaskCommand) -> AppResult<Task>;
    async fn delete_task(&self, task: Uuid) -> AppResult<()>;
}

#[async_trait]
pub trait FocusSessionPersistence: Send + Sync {
    async fn create_manual_session(
        &self,
        session: &CreateManualSessionData,
    ) -> AppResult<FocusSession>;
}
