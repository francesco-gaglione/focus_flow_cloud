use crate::application::app_error::AppResult;
use crate::application::traits::{CategoryPersistence, TaskPersistence};
use crate::application::use_cases::commands::category_with_tasks::{
    CategoryAndTasks, CategoryWithTasks,
};
use crate::application::use_cases::commands::create_category::CreateCategoryCommand;
use crate::application::use_cases::commands::update_category::UpdateCategoryCommand;
use crate::application::use_cases::persistance_command::create_category_data::CreateCategoryData;
use crate::application::use_cases::persistance_command::update_category_data::UpdateCategoryData;
use crate::domain::entities::category::Category;
use rand::Rng;
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

    pub async fn create_category(&self, category: CreateCategoryCommand) -> AppResult<()> {
        self.category_persistence
            .create_category(CreateCategoryData {
                name: category.name.clone(),
                description: category.description.clone(),
                color: category.color.clone().unwrap_or(Self::random_hex_color()),
            })
            .await
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

        Ok(CategoryAndTasks {
            category_with_tasks: categories_with_tasks,
        })
    }

    pub async fn update_category(&self, category: UpdateCategoryCommand) -> AppResult<Category> {
        self.category_persistence
            .update_category(
                category.id,
                UpdateCategoryData {
                    name: category.name,
                    description: category.description,
                    color: category.color,
                },
            )
            .await
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

    // helpers functions

    fn random_hex_color() -> String {
        let mut rng = rand::thread_rng();
        let color: u32 = rng.gen_range(0..0xFFFFFF);
        format!("#{:06X}", color)
    }
}

#[cfg(test)]
mod category_use_cases_tests {
    use crate::{
        application::{
            app_error::AppError,
            traits::{MockCategoryPersistence, MockTaskPersistence},
            use_cases::{
                category_use_cases::CategoryUseCases,
                commands::{
                    create_category::CreateCategoryCommand, update_category::UpdateCategoryCommand,
                },
            },
        },
        domain::entities::{category::Category, task::Task},
    };
    use chrono::NaiveDate;
    use std::sync::Arc;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_category_success() {
        let mut mock_category_persistence = MockCategoryPersistence::new();
        let mock_task_persistence = MockTaskPersistence::new();

        mock_category_persistence
            .expect_create_category()
            .returning(|_| Ok(()));

        let use_case = CategoryUseCases::new(
            Arc::new(mock_category_persistence),
            Arc::new(mock_task_persistence),
        );

        let cmd = CreateCategoryCommand {
            name: "Work".to_string(),
            description: Some("Work related tasks".to_string()),
            color: Some("#FF5733".to_string()),
        };

        let result = use_case.create_category(cmd).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_category_with_random_color() {
        let mut mock_category_persistence = MockCategoryPersistence::new();
        let mock_task_persistence = MockTaskPersistence::new();

        mock_category_persistence
            .expect_create_category()
            .withf(|data| {
                data.name == "Work"
                    && data.description == Some("Work related tasks".to_string())
                    && data.color.starts_with("#")
                    && data.color.len() == 7
            })
            .returning(|_| Ok(()));

        let use_case = CategoryUseCases::new(
            Arc::new(mock_category_persistence),
            Arc::new(mock_task_persistence),
        );

        let cmd = CreateCategoryCommand {
            name: "Work".to_string(),
            description: Some("Work related tasks".to_string()),
            color: None, // Should generate random color
        };

        let result = use_case.create_category(cmd).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_category_persistence_error() {
        let mut mock_category_persistence = MockCategoryPersistence::new();
        let mock_task_persistence = MockTaskPersistence::new();

        mock_category_persistence
            .expect_create_category()
            .returning(|_| Err(AppError::Database("Database error".to_string())));

        let use_case = CategoryUseCases::new(
            Arc::new(mock_category_persistence),
            Arc::new(mock_task_persistence),
        );

        let cmd = CreateCategoryCommand {
            name: "Work".to_string(),
            description: Some("Work related tasks".to_string()),
            color: Some("#FF5733".to_string()),
        };

        let result = use_case.create_category(cmd).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_all_category_and_tasks_success() {
        let mut mock_category_persistence = MockCategoryPersistence::new();
        let mut mock_task_persistence = MockTaskPersistence::new();

        let category_id = Uuid::new_v4();
        let category = Category {
            id: category_id,
            name: "Work".to_string(),
            description: Some("Work related tasks".to_string()),
            color: "#FF5733".to_string(),
        };

        let task1 = Task {
            id: Uuid::new_v4(),
            name: "Task 1".to_string(),
            description: Some("First task".to_string()),
            category_id: Some(category_id),
            scheduled_date: NaiveDate::from_ymd_opt(2025, 12, 20),
            completed_at: None,
        };

        mock_category_persistence
            .expect_find_all()
            .returning(move || Ok(vec![category.clone()]));

        mock_task_persistence
            .expect_find_by_category_id()
            .with(mockall::predicate::eq(category_id))
            .returning(move |_| Ok(vec![task1.clone()]));

        mock_task_persistence
            .expect_find_orphan_tasks()
            .returning(|| Ok(vec![]));

        let use_case = CategoryUseCases::new(
            Arc::new(mock_category_persistence),
            Arc::new(mock_task_persistence),
        );

        let result = use_case.get_all_category_and_tasks().await.unwrap();
        assert_eq!(result.category_with_tasks.len(), 1);
        assert_eq!(result.category_with_tasks[0].tasks.len(), 1);
        assert_eq!(result.orphan_tasks.len(), 0);
    }

    #[tokio::test]
    async fn test_get_all_category_and_tasks_with_orphan_tasks() {
        let mut mock_category_persistence = MockCategoryPersistence::new();
        let mut mock_task_persistence = MockTaskPersistence::new();

        let category_id = Uuid::new_v4();
        let category = Category {
            id: category_id,
            name: "Work".to_string(),
            description: Some("Work related tasks".to_string()),
            color: "#FF5733".to_string(),
        };

        let orphan_task = Task {
            id: Uuid::new_v4(),
            name: "Orphan Task".to_string(),
            description: Some("Task without category".to_string()),
            category_id: None,
            scheduled_date: NaiveDate::from_ymd_opt(2025, 12, 20),
            completed_at: Some(chrono::Utc::now()),
        };

        mock_category_persistence
            .expect_find_all()
            .returning(move || Ok(vec![category.clone()]));

        mock_task_persistence
            .expect_find_by_category_id()
            .with(mockall::predicate::eq(category_id))
            .returning(|_| Ok(vec![]));

        mock_task_persistence
            .expect_find_orphan_tasks()
            .returning(move || Ok(vec![orphan_task.clone()]));

        let use_case = CategoryUseCases::new(
            Arc::new(mock_category_persistence),
            Arc::new(mock_task_persistence),
        );

        let result = use_case.get_all_category_and_tasks().await.unwrap();
        assert_eq!(result.category_with_tasks.len(), 1);
        assert_eq!(result.category_with_tasks[0].tasks.len(), 0);
        assert_eq!(result.orphan_tasks.len(), 1);
    }

    #[tokio::test]
    async fn test_update_category_success() {
        let mut mock_category_persistence = MockCategoryPersistence::new();
        let mock_task_persistence = MockTaskPersistence::new();

        let category_id = Uuid::new_v4();
        let updated_category = Category {
            id: category_id,
            name: "Updated Work".to_string(),
            description: Some("Updated description".to_string()),
            color: "#000000".to_string(),
        };

        mock_category_persistence
            .expect_update_category()
            .with(
                mockall::predicate::eq(category_id),
                mockall::predicate::always(),
            )
            .returning(move |_, _| Ok(updated_category.clone()));

        let use_case = CategoryUseCases::new(
            Arc::new(mock_category_persistence),
            Arc::new(mock_task_persistence),
        );

        let cmd = UpdateCategoryCommand {
            id: category_id,
            name: Some("Updated Work".to_string()),
            description: Some("Updated description".to_string()),
            color: Some("#000000".to_string()),
        };

        let result = use_case.update_category(cmd).await.unwrap();
        assert_eq!(result.name, "Updated Work");
    }

    #[tokio::test]
    async fn test_delete_category_success() {
        let mut mock_category_persistence = MockCategoryPersistence::new();
        let mock_task_persistence = MockTaskPersistence::new();

        let category_id = Uuid::new_v4();

        mock_category_persistence
            .expect_delete_category_by_id()
            .with(mockall::predicate::eq(category_id))
            .returning(|_| Ok(()));

        let use_case = CategoryUseCases::new(
            Arc::new(mock_category_persistence),
            Arc::new(mock_task_persistence),
        );

        let result = use_case.delete_category(category_id).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_categories_success() {
        let mut mock_category_persistence = MockCategoryPersistence::new();
        let mock_task_persistence = MockTaskPersistence::new();

        let ids = vec![Uuid::new_v4(), Uuid::new_v4()];

        mock_category_persistence
            .expect_delete_category_by_id()
            .returning(|_| Ok(()))
            .times(ids.len());

        let use_case = CategoryUseCases::new(
            Arc::new(mock_category_persistence),
            Arc::new(mock_task_persistence),
        );

        let result = use_case.delete_categories(ids.clone()).await.unwrap();
        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn test_delete_categories_partial_failure() {
        let mut mock_category_persistence = MockCategoryPersistence::new();
        let mock_task_persistence = MockTaskPersistence::new();

        let ids = vec![Uuid::new_v4(), Uuid::new_v4()];
        let first_id = ids[0];

        // First call succeeds, second fails
        mock_category_persistence
            .expect_delete_category_by_id()
            .with(mockall::predicate::eq(first_id))
            .returning(|_| Ok(()))
            .times(1);

        mock_category_persistence
            .expect_delete_category_by_id()
            .returning(|_| Err(AppError::NotFound("Category not found".to_string())))
            .times(1);

        let use_case = CategoryUseCases::new(
            Arc::new(mock_category_persistence),
            Arc::new(mock_task_persistence),
        );

        let result = use_case.delete_categories(ids).await;
        assert!(result.is_err());
    }
}
