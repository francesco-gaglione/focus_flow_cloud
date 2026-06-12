use std::sync::Arc;

use crate::shared::traits::persistence_error::PersistenceError;
use crate::tasks::traits::category_persistence::CategoryPersistence;
use domain::tasks::entities::category::Category;
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum GetAllCategoryError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type GetAllCategoryResult<T> = Result<T, GetAllCategoryError>;

#[derive(Debug, Clone)]
pub struct CategoryOutput {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub color: String,
}

impl From<&Category> for CategoryOutput {
    fn from(value: &Category) -> Self {
        Self {
            id: value.id(),
            user_id: value.user_id(),
            name: value.name().to_string(),
            color: value.color().to_string(),
        }
    }
}

#[derive(Clone)]
pub struct GetAllCategoryUseCases {
    category_persistence: Arc<dyn CategoryPersistence>,
}

impl GetAllCategoryUseCases {
    pub fn new(category_persistence: Arc<dyn CategoryPersistence>) -> Self {
        Self {
            category_persistence,
        }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self) -> GetAllCategoryResult<Vec<CategoryOutput>> {
        Ok(self
            .category_persistence
            .find_all()
            .await?
            .iter()
            .map(|c| c.into())
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use uuid::Uuid;

    use crate::{
        tasks::traits::category_persistence::MockCategoryPersistence,
        tasks::use_cases::category::get_all_category_usecase::GetAllCategoryUseCases,
    };
    use domain::tasks::entities::category::Category;

    #[tokio::test]
    async fn test_get_category_usecase() {
        let category = Category::reconstitute(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "Test Category".to_string(),
            "#FFFFFF".to_string(),
        )
        .unwrap();

        let mut mock_persistence = MockCategoryPersistence::new();
        let category_to_return = category.clone();

        mock_persistence
            .expect_find_all()
            .times(1)
            .returning(move || Ok(vec![category_to_return.clone()]));

        let usecase = GetAllCategoryUseCases::new(Arc::new(mock_persistence));
        let result = usecase.execute().await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result[0].name, category.name());
    }
}
