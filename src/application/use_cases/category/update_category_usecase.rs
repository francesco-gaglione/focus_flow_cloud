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
