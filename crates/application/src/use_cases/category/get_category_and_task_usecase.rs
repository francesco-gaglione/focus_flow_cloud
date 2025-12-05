use std::sync::Arc;

use crate::{
    app_error::AppResult,
    use_cases::category::command::category_with_tasks::{CategoryAndTasks, CategoryWithTasks},
};
use domain::traits::{
    category_persistence::CategoryPersistence, task_persistence::TaskPersistence,
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use uuid::Uuid;

    use crate::{
        mocks::{MockCategoryPersistence, MockTaskPersistence},
        use_cases::category::get_category_and_task_usecase::GetCategoryAndTaskUseCases,
    };
    use domain::entities::{category::Category, task::Task};

    #[tokio::test]
    async fn test_get_category_and_task_usecase() {
        let mut category_persistence = MockCategoryPersistence::new();
        let mut task_persistence = MockTaskPersistence::new();
        let category_id = Uuid::new_v4();
        category_persistence.expect_find_all().returning(move || {
            Ok(vec![Category::reconstitute(
                category_id.clone(),
                "Test Category".to_string(),
                None,
                "#FF0000".to_string(),
            )
            .unwrap()])
        });
        task_persistence
            .expect_find_by_category_id()
            .returning(move |_| {
                Ok(vec![Task::reconstitute(
                    Uuid::new_v4(),
                    Some(category_id),
                    "task".to_string(),
                    Some("description".to_string()),
                    None,
                    None,
                )])
            });
        task_persistence
            .expect_find_orphan_tasks()
            .returning(|| Ok(vec![]));

        let usecase = GetCategoryAndTaskUseCases::new(
            Arc::new(category_persistence),
            Arc::new(task_persistence),
        );

        let result = usecase.execute().await;

        assert!(result.is_ok());
    }
}
