use crate::db_models::db_category::NewDbCategory;
use crate::entities::category::Category;
use crate::entities::task::Task;
use crate::repository::category_repository::CategoryRepository;
use crate::repository::repository_error::RepositoryError;
use crate::repository::task_repository::TaskRepository;
use crate::services::service_error::ServiceError;

#[derive(Clone, Debug)]
pub struct CreateCategoryCommand {
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
}

#[derive(Clone, Debug)]
pub struct GetCategoriesAndTaskResult {
    pub categories: Vec<Category>,
    pub orphan_tasks: Vec<Task>,
}

#[derive(Clone, Debug)]
pub struct CategoryService {
    category_repository: CategoryRepository,
    task_repository: TaskRepository,
}

impl CategoryService {
    pub fn new(category_repository: CategoryRepository, task_repository: TaskRepository) -> Self {
        Self {
            category_repository,
            task_repository,
        }
    }

    pub async fn create_category(
        &self,
        create_category_command: CreateCategoryCommand,
    ) -> Result<Category, ServiceError> {
        log::debug!("{:?}", create_category_command);

        let db_category = self
            .category_repository
            .create(NewDbCategory {
                name: create_category_command.name.clone(),
                description: create_category_command.description.clone(),
                color: create_category_command.color.clone(),
            })
            .await
            .map_err(|e| match e {
                RepositoryError::UniqueViolation(_) => {
                    log::warn!("Category '{}' already exists", create_category_command.name);
                    ServiceError::CategoryAlreadyExists
                }
                other => {
                    log::error!("{:?}", other);
                    ServiceError::RepositoryError(other)
                }
            })?;

        let category = Category {
            id: db_category.id,
            name: db_category.name,
            description: db_category.description,
            color: db_category.color,
            tasks: vec![],
        };

        Ok(category)
    }

    pub async fn get_all_categories_and_tasks(
        &self,
    ) -> Result<GetCategoriesAndTaskResult, ServiceError> {
        let db_categories = self.category_repository.find_all().await?;
        let db_tasks = self.task_repository.find_all().await?;

        let categories: Vec<Category> = db_categories
            .iter()
            .map(|c| {
                let mut category: Category = c.into();
                category.tasks = db_tasks
                    .iter()
                    .filter(|t| t.category_id.is_some() && t.category_id.unwrap() == c.id)
                    .map(|t| t.into())
                    .collect();
                category
            })
            .collect();

        Ok(GetCategoriesAndTaskResult {
            categories,
            orphan_tasks: db_tasks
                .iter()
                .filter(|t| t.category_id.is_none())
                .map(|t| t.into())
                .collect(),
        })
    }
}
