use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    application::{
        app_error::AppResult,
        use_cases::persistance_command::{
            create_category_data::CreateCategoryData, update_category_data::UpdateCategoryData,
        },
    },
    domain::entities::category::Category,
};

#[cfg_attr(test, mockall::automock)]
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
