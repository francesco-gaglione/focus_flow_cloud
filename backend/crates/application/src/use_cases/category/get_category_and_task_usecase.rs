use std::sync::Arc;

use crate::persistence_traits::category_persistence::CategoryPersistence;
use crate::persistence_traits::persistence_error::PersistenceError;
use crate::persistence_traits::task_persistence::TaskPersistence;
use crate::use_cases::category::command::category_with_tasks::{
    CategoryAndTasks, CategoryWithTasks,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GetCategoryAndTasksError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type GetCategoryAndTasksResult<T> = Result<T, GetCategoryAndTasksError>;

pub struct GetCategoryAndTasksCommand {
    pub include_completed_tasks: Option<bool>,
}

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

    pub async fn execute(
        &self,
        command: GetCategoryAndTasksCommand,
    ) -> GetCategoryAndTasksResult<CategoryAndTasks> {
        let mut categories = self.category_persistence.find_all().await?;

        let mut categories_with_tasks: Vec<CategoryWithTasks> = Vec::new();

        let include_completed_tasks = command.include_completed_tasks.unwrap_or(false);

        for c in &mut categories {
            let tasks = self.task_persistence.find_by_category_id(c.id()).await?;

            let filtered_tasks = if include_completed_tasks {
                tasks
            } else {
                tasks.into_iter().filter(|t| !t.is_completed()).collect()
            };

            categories_with_tasks.push(CategoryWithTasks {
                category: c.clone(),
                tasks: filtered_tasks,
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
        use_cases::category::get_category_and_task_usecase::{
            GetCategoryAndTaskUseCases, GetCategoryAndTasksCommand,
        },
    };
    use domain::entities::{category::Category, task::Task};

    #[tokio::test]
    async fn test_get_category_and_task_usecase_default_filters() {
        let mut category_persistence = MockCategoryPersistence::new();
        let mut task_persistence = MockTaskPersistence::new();
        let category_id = Uuid::new_v4();

        // Setup Category
        category_persistence.expect_find_all().returning(move || {
            Ok(vec![Category::reconstitute(
                category_id.clone(),
                Uuid::new_v4(),
                "Test Category".to_string(),
                None,
                "#FF0000".to_string(),
            )
            .unwrap()])
        });

        // Setup Tasks (1 active, 1 completed)
        task_persistence
            .expect_find_by_category_id()
            .returning(move |_| {
                let mut completed_task = Task::reconstitute(
                    Uuid::new_v4(),
                    Uuid::new_v4(),
                    Some(category_id),
                    "Completed Task".to_string(),
                    None,
                    None,
                    None,
                );
                completed_task.complete(); // Mark as completed

                Ok(vec![
                    Task::reconstitute(
                        Uuid::new_v4(),
                        Uuid::new_v4(),
                        Some(category_id),
                        "Active Task".to_string(),
                        Some("description".to_string()),
                        None,
                        None,
                    ),
                    completed_task,
                ])
            });

        task_persistence
            .expect_find_orphan_tasks()
            .returning(|_| Ok(vec![]));

        let usecase = GetCategoryAndTaskUseCases::new(
            Arc::new(category_persistence),
            Arc::new(task_persistence),
        );

        // Execute with default (should exclude completed)
        let command = GetCategoryAndTasksCommand {
            include_completed_tasks: None,
        };
        let result = usecase.execute(command).await;

        assert!(result.is_ok());
        let categories = result.unwrap().category_with_tasks;
        assert_eq!(categories.len(), 1);
        // Should only have the active task
        assert_eq!(categories[0].tasks.len(), 1);
        assert_eq!(categories[0].tasks[0].name(), "Active Task");
    }

    #[tokio::test]
    async fn test_get_category_and_task_usecase_include_completed() {
        let mut category_persistence = MockCategoryPersistence::new();
        let mut task_persistence = MockTaskPersistence::new();
        let category_id = Uuid::new_v4();

        category_persistence.expect_find_all().returning(move || {
            Ok(vec![Category::reconstitute(
                category_id.clone(),
                Uuid::new_v4(),
                "Test Category".to_string(),
                None,
                "#FF0000".to_string(),
            )
            .unwrap()])
        });

        task_persistence
            .expect_find_by_category_id()
            .returning(move |_| {
                let mut completed_task = Task::reconstitute(
                    Uuid::new_v4(),
                    Uuid::new_v4(),
                    Some(category_id),
                    "Completed Task".to_string(),
                    None,
                    None,
                    None,
                );
                completed_task.complete();

                Ok(vec![
                    Task::reconstitute(
                        Uuid::new_v4(),
                        Uuid::new_v4(),
                        Some(category_id),
                        "Active Task".to_string(),
                        None,
                        None,
                        None,
                    ),
                    completed_task,
                ])
            });

        task_persistence
            .expect_find_orphan_tasks()
            .returning(|_| Ok(vec![]));

        let usecase = GetCategoryAndTaskUseCases::new(
            Arc::new(category_persistence),
            Arc::new(task_persistence),
        );

        // Execute with include_completed_tasks = true
        let command = GetCategoryAndTasksCommand {
            include_completed_tasks: Some(true),
        };
        let result = usecase.execute(command).await;

        assert!(result.is_ok());
        let categories = result.unwrap().category_with_tasks;
        assert_eq!(categories.len(), 1);
        // Should have both tasks
        assert_eq!(categories[0].tasks.len(), 2);
    }
}
