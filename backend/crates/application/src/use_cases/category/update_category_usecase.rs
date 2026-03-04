use std::sync::Arc;

use crate::persistence_traits::category_persistence::CategoryPersistence;
use crate::persistence_traits::persistence_error::PersistenceError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UpdateCategoryError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type UpdateCategoryResult<T> = Result<T, UpdateCategoryError>;

#[derive(Debug, Clone)]
pub struct UpdateCategoryCommand {
    pub id: uuid::Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
}

#[derive(Clone)]
pub struct UpdateCategoryUseCases {
    category_persistence: Arc<dyn CategoryPersistence>,
}

impl UpdateCategoryUseCases {
    pub fn new(category_persistence: Arc<dyn CategoryPersistence>) -> Self {
        Self {
            category_persistence,
        }
    }

    pub async fn execute(&self, command: UpdateCategoryCommand) -> UpdateCategoryResult<()> {
        let mut category = self.category_persistence.find_by_id(command.id).await?;

        if let Some(name) = command.name {
            category.update_name(name);
        }
        if let Some(description) = command.description {
            category.update_description(Some(description));
        }
        if let Some(color) = command.color {
            category.update_color(color);
        }

        self.category_persistence.update_category(category).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        persistence_traits::category_persistence::MockCategoryPersistence,
        use_cases::category::update_category_usecase::{
            UpdateCategoryCommand, UpdateCategoryUseCases,
        },
    };
    use domain::entities::category::Category;

    #[tokio::test]
    async fn test_update_category_usecase() {
        let category_id = uuid::Uuid::new_v4();
        let category_name = "Test Category".to_string();
        let category_description = "Test Description".to_string();
        let category_color = "#FF0000".to_string();

        let mut category_persistence = MockCategoryPersistence::new();

        let name_clone_update = category_name.clone();
        let description_clone_update = category_description.clone();
        let color_clone_update = category_color.clone();

        let name_clone_find = category_name.clone();
        let description_clone_find = category_description.clone();
        let color_clone_find = category_color.clone();

        category_persistence
            .expect_update_category()
            .returning(move |_| {
                Ok(Category::reconstitute(
                    category_id,
                    uuid::Uuid::new_v4(),
                    name_clone_update.clone(),
                    Some(description_clone_update.clone()),
                    color_clone_update.clone(),
                )
                .unwrap())
            });

        category_persistence
            .expect_find_by_id()
            .returning(move |_| {
                Ok(Category::reconstitute(
                    category_id,
                    uuid::Uuid::new_v4(),
                    name_clone_find.clone(),
                    Some(description_clone_find.clone()),
                    color_clone_find.clone(),
                )
                .unwrap())
            });
        let use_case = UpdateCategoryUseCases::new(Arc::new(category_persistence));

        let result = use_case
            .execute(UpdateCategoryCommand {
                id: category_id,
                name: Some(category_name.clone()),
                description: Some(category_description.clone()),
                color: Some(category_color.clone()),
            })
            .await;

        assert!(result.is_ok())
    }
}
