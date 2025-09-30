use crate::repository::category_repository::CategoryRepository;
use crate::services::category_service::CategoryService;
use deadpool_diesel::postgres::Pool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub category_service: Arc<CategoryService>,
}

impl AppState {
    pub fn new(db_pool: Pool) -> Self {
        let category_repository = CategoryRepository::new(db_pool.clone());

        Self {
            category_service: Arc::new(CategoryService::new(category_repository)),
        }
    }
}
