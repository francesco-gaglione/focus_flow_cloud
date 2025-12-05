use std::sync::Arc;

use crate::app_error::{AppError, AppResult};
use domain::traits::category_persistence::CategoryPersistence;

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

    pub async fn execute(&self, category_id: uuid::Uuid) -> AppResult<()> {
        self.category_persistence
            .delete_category_by_id(category_id)
            .await
            .map_err(AppError::from)
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
