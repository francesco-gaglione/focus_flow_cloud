use std::sync::Arc;

use crate::{
    application::{app_error::AppResult, traits::category_persistence::CategoryPersistence},
    domain::entities::category::Category,
};

#[derive(Clone)]
pub struct GetCategoryUseCases {
    category_persistence: Arc<dyn CategoryPersistence>,
}

impl GetCategoryUseCases {
    pub fn new(category_persistence: Arc<dyn CategoryPersistence>) -> Self {
        Self {
            category_persistence,
        }
    }

    pub async fn execute(&self, category_id: uuid::Uuid) -> AppResult<Category> {
        self.category_persistence.find_by_id(category_id).await
    }
}
