use crate::persistence_traits::persistence_error::PersistenceResult;
use async_trait::async_trait;
use domain::entities::category::Category;
use uuid::Uuid;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait CategoryPersistence: Send + Sync {
    async fn create_category(&self, category: Category) -> PersistenceResult<Uuid>;

    async fn find_all(&self) -> PersistenceResult<Vec<Category>>;

    async fn find_by_id(&self, category_id: Uuid) -> PersistenceResult<Category>;

    async fn update_category(&self, category: Category) -> PersistenceResult<Category>;

    async fn delete_category_by_id(&self, category_id: Uuid) -> PersistenceResult<()>;
}
