use crate::db_models::db_category::NewDbCategory;
use crate::entities::category::Category;
use crate::repository::category_repository::CategoryRepository;
use crate::repository::repository_error::RepositoryError;
use crate::services::service_error::ServiceError;

#[derive(Clone, Debug)]
pub struct CreateCategoryCommand {
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
}

#[derive(Clone, Debug)]
pub struct CategoryService {
    category_repository: CategoryRepository,
}

impl CategoryService {
    pub fn new(category_repository: CategoryRepository) -> Self {
        Self {
            category_repository,
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
        };

        Ok(category)
    }
}
