use std::sync::Arc;

use domain::{
    error::persistence_error::PersistenceError, traits::category_persistence::CategoryPersistence,
};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum DeleteCategoriesError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type DeleteCategoriesResult<T> = Result<T, DeleteCategoriesError>;

#[derive(Clone)]
pub struct DeleteCategoriesUseCases {
    category_persistence: Arc<dyn CategoryPersistence>,
}

impl DeleteCategoriesUseCases {
    pub fn new(category_persistence: Arc<dyn CategoryPersistence>) -> Self {
        Self {
            category_persistence,
        }
    }

    pub async fn execute(
        &self,
        category_ids: Vec<uuid::Uuid>,
    ) -> DeleteCategoriesResult<Vec<uuid::Uuid>> {
        let mut deleted_ids: Vec<uuid::Uuid> = Vec::new();
        for category_id in category_ids {
            self.category_persistence
                .delete_category_by_id(category_id)
                .await?;
            deleted_ids.push(category_id);
        }
        Ok(deleted_ids)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        mocks::MockCategoryPersistence,
        use_cases::category::delete_categories_usecase::DeleteCategoriesUseCases,
    };

    #[tokio::test]
    async fn test_delete_categories_usecase() {
        let mut category_persistence = MockCategoryPersistence::new();
        category_persistence
            .expect_delete_category_by_id()
            .returning(|_| Ok(()));
        let usecase = DeleteCategoriesUseCases::new(Arc::new(category_persistence));

        let category_ids = vec![uuid::Uuid::new_v4(), uuid::Uuid::new_v4()];
        let result = usecase.execute(category_ids.clone()).await;

        assert_eq!(result, Ok(category_ids));
    }
}
