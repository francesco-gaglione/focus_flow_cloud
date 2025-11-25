use std::sync::Arc;

use crate::application::{app_error::AppResult, traits::category_persistence::CategoryPersistence};

#[derive(Clone)]
pub struct DeleteCategoryUseCases {
    category_persistence: Arc<dyn CategoryPersistence>,
}

impl DeleteCategoryUseCases {
    pub fn new(category_persistence: Arc<dyn CategoryPersistence>) -> Self {
        Self {
            category_persistence,
        }
    }

    pub async fn execute(&self, category_id: uuid::Uuid) -> AppResult<()> {
        self.category_persistence
            .delete_category_by_id(category_id)
            .await
    }
}
