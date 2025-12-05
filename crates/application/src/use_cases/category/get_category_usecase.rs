use std::sync::Arc;

use crate::{app_error::AppResult, traits::category_persistence::CategoryPersistence};
use domain::entities::category::Category;

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

    pub async fn execute(&self, category_id: uuid::Uuid) -> AppResult<Category> {
        self.category_persistence.find_by_id(category_id).await
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate;
    use std::sync::Arc;
    use uuid::Uuid;

    use crate::{
        traits::category_persistence::MockCategoryPersistence,
        use_cases::category::get_category_usecase::GetCategoryUseCases,
    };
    use domain::entities::category::Category;

    #[tokio::test]
    async fn test_get_category_usecase() {
        let category_id = Uuid::new_v4();
        let category = Category::new(
            category_id,
            "Test Category".to_string(),
            None,
            "#FFFFFF".to_string(),
        );

        let mut mock_persistence = MockCategoryPersistence::new();
        let category_to_return = category.clone();

        mock_persistence
            .expect_find_by_id()
            .with(predicate::eq(category_id))
            .times(1)
            .returning(move |_| Ok(category_to_return.clone()));

        let usecase = GetCategoryUseCases::new(Arc::new(mock_persistence));
        let result = usecase.execute(category_id).await;

        assert_eq!(result.unwrap(), category);
    }
}
