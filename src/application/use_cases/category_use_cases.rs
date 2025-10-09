use crate::application::app_error::AppResult;
use crate::application::traits::{CategoryPersistence, TaskPersistence};
use crate::application::use_cases::commands::category_with_tasks::{
    CategoryAndTasks, CategoryWithTasks,
};
use crate::application::use_cases::commands::create_category::CreateCategoryCommand;
use crate::application::use_cases::commands::update_category::UpdateCategoryCommand;
use crate::application::use_cases::persistance_command::create_category_data::CreateCategoryData;
use crate::application::use_cases::persistance_command::update_category_data::UpdateCategoryData;
use crate::domain::entities::category::Category;
use rand::Rng;
use std::sync::Arc;

#[derive(Clone)]
pub struct CategoryUseCases {
    category_persistence: Arc<dyn CategoryPersistence>,
    task_persistence: Arc<dyn TaskPersistence>,
}

impl CategoryUseCases {
    pub fn new(
        category_persistence: Arc<dyn CategoryPersistence>,
        task_persistence: Arc<dyn TaskPersistence>,
    ) -> Self {
        Self {
            category_persistence,
            task_persistence,
        }
    }

    pub async fn create_category(&self, category: CreateCategoryCommand) -> AppResult<()> {
        self.category_persistence
            .create_category(CreateCategoryData {
                name: category.name.clone(),
                description: category.description.clone(),
                color: category.color.clone().unwrap_or(Self::random_hex_color()),
            })
            .await
    }

    pub async fn get_all_category_and_tasks(&self) -> AppResult<CategoryAndTasks> {
        let mut categories = self.category_persistence.find_all().await?;

        let mut categories_with_tasks: Vec<CategoryWithTasks> = Vec::new();

        for c in &mut categories {
            let tasks = self.task_persistence.find_by_category_id(c.id).await?;
            categories_with_tasks.push(CategoryWithTasks {
                category: c.clone(),
                tasks,
            });
        }

        let orphan_tasks = self.task_persistence.find_orphan_tasks().await?;

        Ok(CategoryAndTasks {
            category_with_tasks: categories_with_tasks,
            orphan_tasks,
        })
    }

    pub async fn update_category(&self, category: UpdateCategoryCommand) -> AppResult<Category> {
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

    pub async fn delete_category(&self, category_id: uuid::Uuid) -> AppResult<()> {
        self.category_persistence
            .delete_category_by_id(category_id)
            .await
    }

    pub async fn delete_categories(
        &self,
        category_ids: Vec<uuid::Uuid>,
    ) -> AppResult<Vec<uuid::Uuid>> {
        let mut deleted_ids: Vec<uuid::Uuid> = Vec::new();
        for category_id in category_ids {
            self.delete_category(category_id).await?;
            deleted_ids.push(category_id);
        }
        Ok(deleted_ids)
    }

    // helpers functions

    fn random_hex_color() -> String {
        let mut rng = rand::thread_rng();
        let color: u32 = rng.gen_range(0..0xFFFFFF);
        format!("#{:06X}", color)
    }
}

#[cfg(test)]
mod tests {
    //TODO tests implementation asap on first pre alpha version (when models are stabe)
}
