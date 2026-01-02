use async_trait::async_trait;
use uuid::Uuid;

use crate::{entities::category::Category, error::persistence_error::PersistenceResult};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait CategoryPersistence: Send + Sync {
    async fn create_category(&self, category: Category) -> PersistenceResult<Uuid>;

    async fn find_all(&self) -> PersistenceResult<Vec<Category>>;

    async fn find_by_id(&self, category_id: Uuid) -> PersistenceResult<Category>;

    async fn update_category(&self, category: Category) -> PersistenceResult<Category>;

    async fn delete_category_by_id(&self, category_id: Uuid) -> PersistenceResult<()>;
}
