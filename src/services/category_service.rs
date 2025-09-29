use crate::db_models::db_category::NewDbCategory;
use crate::entities::category::Category;
use crate::repository::category_repository::CategoryRepository;
use crate::services::service_error::ServiceError;

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
        let db_category = self
            .category_repository
            .create(NewDbCategory {
                name: create_category_command.name,
                description: create_category_command.description,
                color: create_category_command.color,
            })
            .await
            .map_err(|e| ServiceError::RepositoryError(e.into()))?;

        let category = Category {
            id: db_category.id,
            name: db_category.name,
            description: db_category.description,
            color: db_category.color,
        };

        Ok(category)
    }
}
