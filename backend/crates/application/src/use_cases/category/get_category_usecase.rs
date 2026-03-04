use std::sync::Arc;

use crate::persistence_traits::category_persistence::CategoryPersistence;
use crate::persistence_traits::persistence_error::PersistenceError;
use domain::entities::category::Category;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum GetCategoryError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type GetCategoryResult<T> = Result<T, GetCategoryError>;

#[derive(Debug, Clone)]
pub struct CategoryOutput {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub color: String,
}

impl From<Category> for CategoryOutput {
    fn from(value: Category) -> Self {
        Self {
            id: value.id(),
            user_id: value.user_id(),
            name: value.name().to_string(),
            description: value.description().map(|d| d.to_string()),
            color: value.color().to_string(),
        }
    }
}

#[derive(Clone)]
pub struct GetCategoryUseCases {
    category_persistence: Arc<dyn CategoryPersistence>,
}

impl GetCategoryUseCases {
    pub fn new(category_persistence: Arc<dyn CategoryPersistence>) -> Self {
        Self {
            category_persistence,
        }
    }

    pub async fn execute(&self, category_id: Uuid) -> GetCategoryResult<CategoryOutput> {
        Ok(self
            .category_persistence
            .find_by_id(category_id)
            .await?
            .into())
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate;
    use std::sync::Arc;
    use uuid::Uuid;

    use crate::{
        persistence_traits::category_persistence::MockCategoryPersistence,
        use_cases::category::get_category_usecase::GetCategoryUseCases,
    };
    use domain::entities::category::Category;

    #[tokio::test]
    async fn test_get_category_usecase() {
        let category_id = Uuid::new_v4();
        let category = Category::reconstitute(
            category_id,
            Uuid::new_v4(),
            "Test Category".to_string(),
            None,
            "#FFFFFF".to_string(),
        )
        .unwrap();

        let mut mock_persistence = MockCategoryPersistence::new();
        let category_to_return = category.clone();

        mock_persistence
            .expect_find_by_id()
            .with(predicate::eq(category_id))
            .times(1)
            .returning(move |_| Ok(category_to_return.clone()));

        let usecase = GetCategoryUseCases::new(Arc::new(mock_persistence));
        let result = usecase.execute(category_id).await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.name, category.name());
    }
}
