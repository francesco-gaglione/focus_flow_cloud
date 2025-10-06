use crate::adapters::persistence::PostgresPersistence;
use crate::adapters::persistence::db_models::db_category::{
    DbCategory, NewDbCategory, UpdateDbCategory,
};
use crate::application::app_error::{AppError, AppResult};
use crate::application::traits::CategoryPersistence;
use crate::application::use_cases::commands::create_category::CreateCategoryCommand;
use crate::application::use_cases::commands::update_category::UpdateCategoryCommand;
use crate::domain::entities::category::Category;
use async_trait::async_trait;
use chrono::Utc;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use tracing::{info, warn};
use uuid::Uuid;
use crate::adapters::schema;

#[async_trait]
impl CategoryPersistence for PostgresPersistence {
    async fn create_category(&self, create_command: &CreateCategoryCommand) -> AppResult<()> {
        let category = create_command.clone();
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let result = conn
            .interact(move |conn| {
                diesel::insert_into(schema::categories::table)
                    .values(NewDbCategory::from(category))
                    .returning(DbCategory::as_returning())
                    .get_result(conn)
            })
            .await
            .map_err(|e| AppError::Database("Category not created".to_string()))??;

        info!("Created category with id: {}", result.id);

        Ok(())
    }

    async fn find_all(&self) -> AppResult<Vec<Category>> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let result = conn
            .interact(move |conn| {
                schema::categories::table
                    .filter(schema::categories::deleted_at.is_null())
                    .select(DbCategory::as_select())
                    .order(schema::categories::created_at.desc())
                    .load(conn)
            })
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .map_err(|e| AppError::Database(e.to_string()))?;

        let categories: Vec<Category> = result.into_iter().map(|c| c.into()).collect();

        Ok(categories)
    }

    async fn update_category(&self, update_command: &UpdateCategoryCommand) -> AppResult<Category> {
        let category = update_command.clone();
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let result = conn
            .interact(move |conn| {
                diesel::update(schema::categories::table)
                    .filter(schema::categories::id.eq(category.id))
                    .filter(schema::categories::deleted_at.is_null())
                    .set((
                        &UpdateDbCategory::from(category),
                        schema::categories::updated_at.eq(Utc::now()), // Manual updated_at handling
                    ))
                    .returning(DbCategory::as_returning())
                    .get_result(conn)
                    .optional()
            })
            .await
            .map_err(|e| AppError::Database(e.to_string()))??;

        match result {
            None => Err(AppError::GenericError("Category not updated".to_string())),
            Some(updated) => Ok(updated.into()),
        }
    }

    async fn delete_category_by_id(&self, category_id: Uuid) -> AppResult<()> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let affected_rows = conn
            .interact(move |conn| {
                diesel::delete(schema::categories::table)
                    .filter(schema::categories::id.eq(category_id))
                    .execute(conn)
            })
            .await
            .map_err(|e| AppError::Database(e.to_string()))??;

        match affected_rows > 0 {
            true => Ok(()),
            false => Err(AppError::Database("Category not deleted".to_string())),
        }
    }
}
