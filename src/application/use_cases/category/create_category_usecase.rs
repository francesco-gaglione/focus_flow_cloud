use std::sync::Arc;

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

    pub async fn execute(&self, category: CreateCategoryCommand) -> AppResult<()> {
        self.category_persistence
            .create_category(CreateCategoryData {
                name: category.name.clone(),
                description: category.description.clone(),
                color: category.color.clone().unwrap_or(random_hex_color()),
            })
            .await
    }
}
