use crate::application::app_error::AppResult;
use crate::application::traits::{CategoryPersistence, TaskPersistence};
use crate::application::use_cases::commands::category_with_tasks::{
    CategoryAndTasks, CategoryWithTasks,
};
use crate::application::use_cases::commands::create_category::CreateCategoryCommand;
use crate::application::use_cases::commands::update_category::UpdateCategoryCommand;
use crate::domain::entities::category::Category;
use std::sync::Arc;

#[derive(Clone)]
pub struct CategoryUseCases {
    category_persistence: Arc<dyn CategoryPersistence>,
    task_persistence: Arc<dyn TaskPersistence>,
}

impl CategoryUseCases {
    pub fn new(
        category_persistence: Arc<dyn CategoryPersistence>,
        task_persistence: Arc<dyn TaskPersistence>,
    ) -> Self {
        Self {
            category_persistence,
            task_persistence,
        }
    }

    pub async fn create_category(&self, category: &CreateCategoryCommand) -> AppResult<()> {
        self.category_persistence.create_category(&category).await
    }

    pub async fn get_all_category_and_tasks(&self) -> AppResult<CategoryAndTasks> {
        let mut categories = self.category_persistence.find_all().await?;

        let mut categories_with_tasks: Vec<CategoryWithTasks> = Vec::new();

        for c in &mut categories {
            let tasks = self.task_persistence.find_by_category_id(c.id).await?;
            categories_with_tasks.push(CategoryWithTasks {
                category: c.clone(),
                tasks,
            });
        }

        let orphan_tasks = self.task_persistence.find_orphan_tasks().await?;

        Ok(CategoryAndTasks {
            category_with_tasks: categories_with_tasks,
            orphan_tasks,
        })
    }

    pub async fn update_category(&self, category: &UpdateCategoryCommand) -> AppResult<Category> {
        self.category_persistence.update_category(&category).await
    }

    pub async fn delete_category(&self, category_id: uuid::Uuid) -> AppResult<()> {
        self.category_persistence
            .delete_category_by_id(category_id)
            .await
    }

    pub async fn delete_categories(
        &self,
        category_ids: Vec<uuid::Uuid>,
    ) -> AppResult<Vec<uuid::Uuid>> {
        let mut deleted_ids: Vec<uuid::Uuid> = Vec::new();
        for category_id in category_ids {
            self.delete_category(category_id).await?;
            deleted_ids.push(category_id);
        }
        Ok(deleted_ids)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::app_error::AppError;
    use crate::application::use_cases::commands::create_task::CreateTaskCommand;
    use crate::application::use_cases::commands::update_task::UpdateTaskCommand;
    use crate::domain::entities::category::Category;
    use crate::domain::entities::task::Task;
    use async_trait::async_trait;
    use chrono::NaiveDate;
    use std::collections::HashMap;
    use uuid::Uuid;

    struct MockCategoryPersistence {
        categories: Vec<Category>,
        should_fail: Option<AppError>,
    }

    struct MockTaskPersistence {
        tasks_by_category: HashMap<uuid::Uuid, Vec<Task>>,
        should_fail: bool,
    }

    #[async_trait]
    impl CategoryPersistence for MockCategoryPersistence {
        async fn create_category(&self, _category: &CreateCategoryCommand) -> AppResult<()> {
            if let Some(err) = &self.should_fail {
                return Err(err.clone());
            }
            Ok(())
        }

        async fn find_all(&self) -> AppResult<Vec<Category>> {
            Ok(self.categories.clone())
        }

        async fn update_category(&self, category: &UpdateCategoryCommand) -> AppResult<Category> {
            if let Some(err) = &self.should_fail {
                return Err(err.clone());
            }
            let category = Category {
                id: category.id.clone(),
                name: "updated".to_string(),
                description: category.description.clone(),
                color: category.color.clone(),
            };
            Ok(category)
        }

        async fn delete_category_by_id(&self, category_id: Uuid) -> AppResult<()> {
            Ok(())
        }
    }

    #[async_trait]
    impl TaskPersistence for MockTaskPersistence {
        async fn find_all(&self) -> AppResult<Vec<Task>> {
            Ok(self.tasks_by_category.values().flatten().cloned().collect())
        }

        async fn create_task(&self, _task: &CreateTaskCommand) -> AppResult<Uuid> {
            Ok(Uuid::new_v4())
        }

        async fn find_by_category_id(&self, category_id: Uuid) -> AppResult<Vec<Task>> {
            if self.should_fail {
                return Err(AppError::GenericError("Failed to fetch tasks".to_string()));
            }
            Ok(self
                .tasks_by_category
                .get(&category_id)
                .cloned()
                .unwrap_or_default())
        }

        async fn update_task(&self, task: &UpdateTaskCommand) -> AppResult<Task> {
            Ok(Task {
                id: task.id.clone(),
                category_id: task.category_id.clone(),
                name: "updated".to_string(),
                description: task.description.clone(),
                scheduled_date: task.scheduled_date.clone(),
                completed_at: task.completed_at.clone(),
            })
        }

        async fn delete_task(&self, task: Uuid) -> AppResult<()> {
            Ok(())
        }

        async fn find_orphan_tasks(&self) -> AppResult<Vec<Task>> {
            Ok(Vec::new())
        }
    }

    fn create_test_category(id: uuid::Uuid, name: &str) -> Category {
        Category {
            id,
            name: name.to_string(),
            description: None,
            color: None,
        }
    }

    fn create_test_task(name: &str, day: u32) -> Task {
        Task {
            id: uuid::Uuid::new_v4(),
            category_id: None,
            name: name.to_string(),
            description: None,
            scheduled_date: NaiveDate::from_ymd_opt(2025, 10, day),
            completed_at: None,
        }
    }

    #[tokio::test]
    async fn test_create_category_success() {
        let use_cases = CategoryUseCases::new(
            Arc::new(MockCategoryPersistence {
                categories: vec![],
                should_fail: None,
            }),
            Arc::new(MockTaskPersistence {
                tasks_by_category: HashMap::new(),
                should_fail: false,
            }),
        );

        let result = use_cases
            .create_category(&CreateCategoryCommand {
                name: "test".to_string(),
                description: None,
                color: None,
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_category_already_exists() {
        let use_cases = CategoryUseCases::new(
            Arc::new(MockCategoryPersistence {
                categories: vec![],
                should_fail: Some(AppError::ResourceAlreadyExist(
                    "Category already exists".to_string(),
                )),
            }),
            Arc::new(MockTaskPersistence {
                tasks_by_category: HashMap::new(),
                should_fail: false,
            }),
        );

        let result = use_cases
            .create_category(&CreateCategoryCommand {
                name: "duplicate".to_string(),
                description: None,
                color: None,
            })
            .await;

        assert!(matches!(result, Err(AppError::ResourceAlreadyExist(_))));
    }

    #[tokio::test]
    async fn test_get_all_categories_with_tasks() {
        let cat1_id = uuid::Uuid::new_v4();
        let cat2_id = uuid::Uuid::new_v4();

        let mut tasks_map = HashMap::new();
        tasks_map.insert(
            cat1_id,
            vec![
                create_test_task("Task 1", 11),
                create_test_task("Task 2", 12),
            ],
        );
        tasks_map.insert(cat2_id, vec![create_test_task("Task 3", 13)]);

        let use_cases = CategoryUseCases::new(
            Arc::new(MockCategoryPersistence {
                categories: vec![
                    create_test_category(cat1_id, "Work"),
                    create_test_category(cat2_id, "Personal"),
                ],
                should_fail: None,
            }),
            Arc::new(MockTaskPersistence {
                tasks_by_category: tasks_map,
                should_fail: false,
            }),
        );

        let result = use_cases.get_all_category_and_tasks().await.unwrap();

        assert_eq!(result.category_with_tasks.len(), 2);
        assert_eq!(result.category_with_tasks[0].tasks.len(), 2);
        assert_eq!(result.category_with_tasks[1].tasks.len(), 1);
    }

    #[tokio::test]
    async fn test_get_all_categories_task_fetch_fails() {
        let use_cases = CategoryUseCases::new(
            Arc::new(MockCategoryPersistence {
                categories: vec![create_test_category(uuid::Uuid::new_v4(), "Test")],
                should_fail: None,
            }),
            Arc::new(MockTaskPersistence {
                tasks_by_category: HashMap::new(),
                should_fail: true,
            }),
        );

        let result = use_cases.get_all_category_and_tasks().await;

        assert!(matches!(result, Err(AppError::GenericError(_))));
    }

    #[tokio::test]
    async fn test_delete_category_success() {
        let use_cases = CategoryUseCases::new(
            Arc::new(MockCategoryPersistence {
                categories: vec![],
                should_fail: None,
            }),
            Arc::new(MockTaskPersistence {
                tasks_by_category: HashMap::new(),
                should_fail: false,
            }),
        );

        let result = use_cases.delete_category(uuid::Uuid::new_v4()).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_category_not_found() {
        let use_cases = CategoryUseCases::new(
            Arc::new(MockCategoryPersistence {
                categories: vec![],
                should_fail: Some(AppError::NotFound("Category not found".to_string())),
            }),
            Arc::new(MockTaskPersistence {
                tasks_by_category: HashMap::new(),
                should_fail: false,
            }),
        );

        let result = use_cases.delete_category(uuid::Uuid::new_v4()).await;

        assert!(matches!(result, Err(AppError::NotFound(_))));
    }
}
