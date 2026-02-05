use std::sync::Arc;

use domain::{
    error::persistence_error::PersistenceError, traits::category_persistence::CategoryPersistence,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeleteCategoryError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type DeleteCategoryResult<T> = Result<T, DeleteCategoryError>;

#[derive(Clone)]
pub struct DeleteCategoryUseCases {
    category_persistence: Arc<dyn CategoryPersistence>,
}

impl DeleteCategoryUseCases {
    pub fn new(category_persistence: Arc<dyn CategoryPersistence>) -> Self {
        Self {
            category_persistence,
        }
    }

    pub async fn execute(&self, category_id: uuid::Uuid) -> DeleteCategoryResult<()> {
        self.category_persistence
            .delete_category_by_id(category_id)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks::MockCategoryPersistence;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_delete_category_usecase() {
        let mut category_persistence = MockCategoryPersistence::new();
        let category_id = uuid::Uuid::new_v4();

        category_persistence
            .expect_delete_category_by_id()
            .with(eq(category_id))
            .once()
            .returning(|_| Ok(()));

        let usecase = DeleteCategoryUseCases::new(Arc::new(category_persistence));

        let result = usecase.execute(category_id).await;

        assert!(result.is_ok());
    }
}
