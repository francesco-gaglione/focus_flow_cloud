use std::sync::Arc;

use crate::{
    application::{
        app_error::AppResult,
        traits::category_persistence::CategoryPersistence,
        use_cases::{
            category::command::update_category::UpdateCategoryCommand,
            persistance_command::update_category_data::UpdateCategoryData,
        },
    },
    domain::entities::category::Category,
};

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

    pub async fn execute(&self, category: UpdateCategoryCommand) -> AppResult<Category> {
        self.category_persistence
            .update_category(
                category.id,
                UpdateCategoryData {
                    name: category.name,
                    description: category.description,
                    color: category.color,
                },
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        application::{
            traits::category_persistence::MockCategoryPersistence,
            use_cases::category::{
                command::update_category::UpdateCategoryCommand,
                update_category_usecase::UpdateCategoryUseCases,
            },
        },
        domain::entities::category::Category,
    };

    #[tokio::test]
    async fn test_update_category_usecase() {
        let category_id = uuid::Uuid::new_v4();
        let category_name = "Test Category".to_string();
        let category_description = "Test Description".to_string();
        let category_color = "#FF0000".to_string();

        let mut category_persistence = MockCategoryPersistence::new();

        // Clone variables for the closure
        let name_clone = category_name.clone();
        let description_clone = category_description.clone();
        let color_clone = category_color.clone();

        category_persistence
            .expect_update_category()
            .returning(move |_, _| {
                Ok(Category::new(
                    category_id,
                    name_clone.clone(),
                    Some(description_clone.clone()),
                    color_clone.clone(),
                ))
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

        let updated_category = result.unwrap();
        assert_eq!(updated_category.name(), category_name);
        assert_eq!(
            updated_category.description(),
            Some(category_description.as_str())
        );
        assert_eq!(updated_category.color(), category_color);
    }
}
