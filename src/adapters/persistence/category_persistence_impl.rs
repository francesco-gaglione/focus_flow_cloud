use crate::adapters::persistence::db_models::db_category::{
    DbCategory, NewDbCategory, UpdateDbCategory,
};
use crate::adapters::persistence::PostgresPersistence;
use crate::adapters::schema;
use crate::application::app_error::{AppError, AppResult};
use crate::application::traits::category_persistence::CategoryPersistence;
use crate::application::use_cases::persistance_command::create_category_data::CreateCategoryData;
use crate::application::use_cases::persistance_command::update_category_data::UpdateCategoryData;
use crate::domain::entities::category::Category;
use async_trait::async_trait;
use chrono::Utc;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use tracing::{error, info};
use uuid::Uuid;

#[async_trait]
impl CategoryPersistence for PostgresPersistence {
    async fn create_category(&self, create_data: CreateCategoryData) -> AppResult<()> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let result = conn
            .interact(move |conn| {
                diesel::insert_into(schema::categories::table)
                    .values(NewDbCategory::from(create_data))
                    .returning(DbCategory::as_returning())
                    .get_result(conn)
            })
            .await
            .map_err(|e| {
                error!("Failed to create category: {}", e);
                AppError::Database("Category not created".to_string())
            })??;

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

    async fn find_by_id(&self, category_id: Uuid) -> AppResult<Category> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let result = conn
            .interact(move |conn| {
                schema::categories::table
                    .filter(schema::categories::id.eq(category_id))
                    .filter(schema::categories::deleted_at.is_null())
                    .select(DbCategory::as_select())
                    .first(conn)
                    .optional()
            })
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .map_err(|e| AppError::Database(e.to_string()))?;

        match result {
            Some(db_category) => Ok(db_category.into()),
            None => Err(AppError::NotFound(format!(
                "Category with id {} not found",
                category_id
            ))),
        }
    }

    async fn update_category(
        &self,
        category_id: Uuid,
        update_command: UpdateCategoryData,
    ) -> AppResult<Category> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let result = conn
            .interact(move |conn| {
                diesel::update(schema::categories::table)
                    .filter(schema::categories::id.eq(category_id))
                    .filter(schema::categories::deleted_at.is_null())
                    .set((
                        &UpdateDbCategory::from(update_command),
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
