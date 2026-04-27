use std::sync::Arc;

use crate::repository_traits::category_persistence::CategoryPersistence;
use crate::repository_traits::persistence_error::PersistenceError;
use crate::repository_traits::task_persistence::TaskPersistence;
use chrono::{DateTime, Utc};
use domain::entities::{category::Category, tasks::task::Task};
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum GetCategoryAndTasksError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type GetCategoryAndTasksResult<T> = Result<T, GetCategoryAndTasksError>;

#[derive(Debug)]
pub struct GetCategoryAndTasksCommand {
    pub include_completed_tasks: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct GetCategoryAndTaskDto {
    pub categories: Vec<CategoryDto>,
    pub tasks: Vec<TaskDto>,
}

#[derive(Debug, Clone)]
pub struct CategoryDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub color: String,
}

#[derive(Debug, Clone)]
pub struct TaskDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl From<&Category> for CategoryDto {
    fn from(value: &Category) -> Self {
        Self {
            id: value.id(),
            user_id: value.user_id(),
            name: value.name().to_string(),
            description: value.description().map(|d| d.to_string()),
            color: value.color().to_string(),
        }
    }
}

impl From<&Task> for TaskDto {
    fn from(value: &Task) -> Self {
        Self {
            id: value.id(),
            user_id: value.user_id(),
            title: value.title().to_string(),
            description: value.description().map(|d| d.to_string()),
            due_date: value.due_date(),
            completed_at: value.completed_at(),
        }
    }
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

    #[instrument(skip(self))]
    pub async fn execute(
        &self,
        command: GetCategoryAndTasksCommand,
    ) -> GetCategoryAndTasksResult<GetCategoryAndTaskDto> {
        let categories = self.category_persistence.find_all().await?;
        let include_completed = command.include_completed_tasks.unwrap_or(false);

        let mut tasks = self.task_persistence.find_all(false).await?;
        if include_completed {
            let completed = self.task_persistence.find_all(true).await?;
            tasks.extend(completed);
        }

        Ok(GetCategoryAndTaskDto {
            categories: categories.iter().map(|c| c.into()).collect(),
            tasks: tasks.iter().map(|t| t.into()).collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use uuid::Uuid;

    use crate::{
        repository_traits::{
            category_persistence::MockCategoryPersistence, task_persistence::MockTaskPersistence,
        },
        use_cases::category::get_category_and_task_usecase::{
            GetCategoryAndTaskUseCases, GetCategoryAndTasksCommand,
        },
    };
    use domain::entities::{category::Category, tasks::task::Task};

    #[tokio::test]
    async fn test_get_category_and_task_usecase_default_filters() {
        let mut category_persistence = MockCategoryPersistence::new();
        let mut task_persistence = MockTaskPersistence::new();

        category_persistence.expect_find_all().returning(move || {
            Ok(vec![Category::reconstitute(
                Uuid::new_v4(),
                Uuid::new_v4(),
                "Test Category".to_string(),
                None,
                "#FF0000".to_string(),
            )
            .unwrap()])
        });

        task_persistence
            .expect_find_all()
            .with(mockall::predicate::eq(false))
            .returning(|_| {
                Ok(vec![Task::new(
                    Uuid::new_v4(),
                    "Active Task".to_string(),
                    None,
                    Some("description".to_string()),
                )])
            });

        let usecase = GetCategoryAndTaskUseCases::new(
            Arc::new(category_persistence),
            Arc::new(task_persistence),
        );

        let result = usecase
            .execute(GetCategoryAndTasksCommand {
                include_completed_tasks: None,
            })
            .await;

        assert!(result.is_ok());
        let dto = result.unwrap();
        assert_eq!(dto.categories.len(), 1);
        assert_eq!(dto.tasks.len(), 1);
        assert_eq!(dto.tasks[0].title, "Active Task");
    }

    #[tokio::test]
    async fn test_get_category_and_task_usecase_include_completed() {
        let mut category_persistence = MockCategoryPersistence::new();
        let mut task_persistence = MockTaskPersistence::new();

        category_persistence.expect_find_all().returning(|| Ok(vec![]));

        task_persistence
            .expect_find_all()
            .with(mockall::predicate::eq(false))
            .returning(|_| {
                Ok(vec![Task::new(
                    Uuid::new_v4(),
                    "Active Task".to_string(),
                    None,
                    None,
                )])
            });

        task_persistence
            .expect_find_all()
            .with(mockall::predicate::eq(true))
            .returning(|_| {
                Ok(vec![Task::new(
                    Uuid::new_v4(),
                    "Completed Task".to_string(),
                    None,
                    None,
                )])
            });

        let usecase = GetCategoryAndTaskUseCases::new(
            Arc::new(category_persistence),
            Arc::new(task_persistence),
        );

        let result = usecase
            .execute(GetCategoryAndTasksCommand {
                include_completed_tasks: Some(true),
            })
            .await;

        assert!(result.is_ok());
        let dto = result.unwrap();
        assert_eq!(dto.tasks.len(), 2);
    }
}
