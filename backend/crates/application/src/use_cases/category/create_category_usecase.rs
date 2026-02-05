use std::sync::Arc;

use thiserror::Error;
use uuid::Uuid;

use domain::{
    entities::category::{Category, CategoryError},
    error::persistence_error::PersistenceError,
    helpers::random_hex_color,
    traits::category_persistence::CategoryPersistence,
};

#[derive(Debug, Error)]
pub enum CreateCategoryError {
    #[error("Category error")]
    CategoryError(#[from] CategoryError),

    #[error("Persistence error")]
    PersistenceError(#[from] PersistenceError),
}

pub type CreateCategoryResult<T> = Result<T, CreateCategoryError>;

pub struct CreateCategoryCommand {
    pub user_id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
}

#[derive(Clone)]
pub struct CreateCategoryUseCases {
    category_persistence: Arc<dyn CategoryPersistence>,
}

impl CreateCategoryUseCases {
    pub fn new(category_persistence: Arc<dyn CategoryPersistence>) -> Self {
        Self {
            category_persistence,
        }
    }

    pub async fn execute(&self, category_cmd: CreateCategoryCommand) -> CreateCategoryResult<Uuid> {
        let category = Category::create(
            category_cmd.user_id,
            category_cmd.name,
            category_cmd.description,
            category_cmd.color.unwrap_or_else(random_hex_color),
        )?;

        self.category_persistence
            .create_category(category.clone())
            .await?;

        Ok(category.id())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use uuid::Uuid;

    use crate::{
        mocks::MockCategoryPersistence,
        use_cases::category::create_category_usecase::{
            CreateCategoryCommand, CreateCategoryUseCases,
        },
    };

    #[tokio::test]
    async fn test_create_category() {
        let mut mock = MockCategoryPersistence::new();
        let name = "Test Category".to_string();
        let description: Option<String> = None;
        let color = "#FF0000".to_string();

        let id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        mock.expect_create_category()
            .withf(move |c| {
                c.name() == "Test Category" && c.color() == "#FF0000" && c.user_id() == user_id
            })
            .returning(move |_| Ok(id));

        let use_cases = CreateCategoryUseCases::new(Arc::new(mock));
        let result = use_cases
            .execute(CreateCategoryCommand {
                user_id,
                name: name.to_string(),
                description: description.map(|d| d.to_string()),
                color: Some(color.to_string()),
            })
            .await;
        assert!(result.is_ok());
    }
}
