use crate::repository::category_repository::CategoryRepository;
use crate::repository::task_repository::TaskRepository;
use crate::services::category_service::CategoryService;
use crate::services::task_service::TaskService;
use deadpool_diesel::postgres::Pool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub category_service: Arc<CategoryService>,
    pub task_service: Arc<TaskService>,
}

impl AppState {
    pub fn new(db_pool: Pool) -> Self {
        let category_repository = CategoryRepository::new(db_pool.clone());
        let task_repository = TaskRepository::new(db_pool.clone());

        Self {
            category_service: Arc::new(CategoryService::new(
                category_repository,
                task_repository.clone(),
            )),
            task_service: Arc::new(TaskService::new(task_repository)),
        }
    }
}
