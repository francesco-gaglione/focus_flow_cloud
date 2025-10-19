use crate::application::app_error::AppResult;
use crate::application::use_cases::persistance_command::create_category_data::CreateCategoryData;
use crate::application::use_cases::persistance_command::create_focus_session_data::CreateSessionData;
use crate::application::use_cases::persistance_command::create_manual_session_data::CreateManualSessionData;
use crate::application::use_cases::persistance_command::create_task_data::CreateTaskData;
use crate::application::use_cases::persistance_command::find_session_by_filters_data::FindSessionByFiltersData;
use crate::application::use_cases::persistance_command::update_category_data::UpdateCategoryData;
use crate::application::use_cases::persistance_command::update_task_data::UpdateTaskData;
use crate::domain::entities::category::Category;
use crate::domain::entities::focus_session::FocusSession;
use crate::domain::entities::task::Task;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait CategoryPersistence: Send + Sync {
    async fn create_category(&self, category: CreateCategoryData) -> AppResult<()>;
    async fn find_all(&self) -> AppResult<Vec<Category>>;
    async fn find_by_id(&self, category_id: Uuid) -> AppResult<Category>;
    async fn update_category(
        &self,
        category_id: Uuid,
        category: UpdateCategoryData,
    ) -> AppResult<Category>;
    async fn delete_category_by_id(&self, category_id: Uuid) -> AppResult<()>;
}

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

#[async_trait]
pub trait FocusSessionPersistence: Send + Sync {
    async fn find_by_filters(
        &self,
        filters: FindSessionByFiltersData,
    ) -> AppResult<Vec<FocusSession>>;

    async fn create_manual_session(
        &self,
        session: &CreateManualSessionData,
    ) -> AppResult<FocusSession>;

    async fn create_session(&self, session: CreateSessionData) -> AppResult<FocusSession>;
}
