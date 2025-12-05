use std::sync::Arc;

use crate::{
    app_error::AppResult, use_cases::category::command::update_category::UpdateCategoryCommand,
};
use domain::entities::category::Category;
use domain::traits::category_persistence::CategoryPersistence;

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

    pub async fn execute(&self, command: UpdateCategoryCommand) -> AppResult<Category> {
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

        Ok(self.category_persistence.update_category(category).await?)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        mocks::MockCategoryPersistence,
        use_cases::category::{
            command::update_category::UpdateCategoryCommand,
            update_category_usecase::UpdateCategoryUseCases,
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

        // Clone variables for the closure
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

        let updated_category = result.unwrap();
        assert_eq!(updated_category.name(), category_name);
        assert_eq!(
            updated_category.description(),
            Some(category_description.as_str())
        );
        assert_eq!(updated_category.color(), category_color);
    }
}
