use std::sync::Arc;

use uuid::Uuid;

use crate::application::{
    app_error::AppResult,
    helpers::random_hex_color,
    traits::category_persistence::CategoryPersistence,
    use_cases::{
        category::command::create_category::CreateCategoryCommand,
        persistance_command::create_category_data::CreateCategoryData,
    },
};

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

    pub async fn execute(&self, category: CreateCategoryCommand) -> AppResult<Uuid> {
        self.category_persistence
            .create_category(CreateCategoryData {
                name: category.name.clone(),
                description: category.description.clone(),
                color: category.color.clone().unwrap_or(random_hex_color()),
            })
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use mockall::predicate;
    use uuid::Uuid;

    use crate::{
        application::{
            traits::category_persistence::MockCategoryPersistence,
            use_cases::category::command::create_category::CreateCategoryCommand,
            use_cases::category::create_category_usecase::CreateCategoryUseCases,
            use_cases::persistance_command::create_category_data::CreateCategoryData,
        },
        domain::entities::category::Category,
    };

    #[tokio::test]
    async fn test_create_category() {
        let mut mock = MockCategoryPersistence::new();
        let category = Category::new(
            Uuid::new_v4(),
            "Test Category".to_string(),
            None,
            "#FF0000".to_string(),
        );
        let id = Uuid::new_v4();
        mock.expect_create_category()
            .with(predicate::eq(CreateCategoryData {
                name: category.name().to_string(),
                description: category.description().map(|d| d.to_string()),
                color: category.color().to_string(),
            }))
            .returning(move |_| Ok(id.clone()));
        let use_cases = CreateCategoryUseCases::new(Arc::new(mock));
        let result = use_cases
            .execute(CreateCategoryCommand {
                name: category.name().to_string(),
                description: category.description().map(|d| d.to_string()),
                color: Some(category.color().to_string()),
            })
            .await;
        assert_eq!(result, Ok(id));
    }
}
