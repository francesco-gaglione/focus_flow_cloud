use std::sync::Arc;

use crate::application::{
    app_error::AppResult,
    traits::{category_persistence::CategoryPersistence, task_persistence::TaskPersistence},
    use_cases::category::command::category_with_tasks::{CategoryAndTasks, CategoryWithTasks},
};

#[derive(Clone)]
pub struct GetCategoryAndTaskUseCases {
    category_persistence: Arc<dyn CategoryPersistence>,
    task_persistence: Arc<dyn TaskPersistence>,
}

impl GetCategoryAndTaskUseCases {
    pub fn new(
        category_persistence: Arc<dyn CategoryPersistence>,
        task_persistence: Arc<dyn TaskPersistence>,
    ) -> Self {
        Self {
            category_persistence,
            task_persistence,
        }
    }

    pub async fn execute(&self) -> AppResult<CategoryAndTasks> {
        let mut categories = self.category_persistence.find_all().await?;

        let mut categories_with_tasks: Vec<CategoryWithTasks> = Vec::new();

        for c in &mut categories {
            let tasks = self.task_persistence.find_by_category_id(c.id()).await?;
            categories_with_tasks.push(CategoryWithTasks {
                category: c.clone(),
                tasks,
            });
        }

        Ok(CategoryAndTasks {
            category_with_tasks: categories_with_tasks,
        })
    }
}
