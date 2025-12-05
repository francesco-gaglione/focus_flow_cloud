use crate::adapters::persistence::db_models::db_category::{
    DbCategory, NewDbCategory, UpdateDbCategory,
};
use crate::adapters::persistence::PostgresPersistence;
use crate::adapters::schema;
use async_trait::async_trait;
use chrono::Utc;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use domain::entities::category::Category;
use domain::error::persistence_error::{PersistenceError, PersistenceResult};
use domain::traits::category_persistence::CategoryPersistence;
use tracing::{error, info};
use uuid::Uuid;

#[async_trait]
impl CategoryPersistence for PostgresPersistence {
    async fn create_category(&self, category: Category) -> PersistenceResult<Uuid> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let result = conn
            .interact(move |conn| {
                diesel::insert_into(schema::categories::table)
                    .values(NewDbCategory::from(category))
                    .returning(DbCategory::as_returning())
                    .get_result(conn)
            })
            .await
            .map_err(|e| {
                error!("Failed to create category: {}", e);
                PersistenceError::Unexpected("Category not created".to_string())
            })?
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        info!("Created category with id: {}", result.id);

        Ok(result.id)
    }

    async fn find_all(&self) -> PersistenceResult<Vec<Category>> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let result = conn
            .interact(move |conn| {
                schema::categories::table
                    .filter(schema::categories::deleted_at.is_null())
                    .select(DbCategory::as_select())
                    .order(schema::categories::created_at.desc())
                    .load(conn)
            })
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let categories: Vec<Category> = result
            .into_iter()
            .map(|c| c.try_into())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(categories)
    }

    async fn find_by_id(&self, category_id: Uuid) -> PersistenceResult<Category> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

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
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        match result {
            Some(db_category) => Ok(db_category.try_into()?),
            None => Err(PersistenceError::NotFound(format!(
                "Category with id {} not found",
                category_id
            ))),
        }
    }

    async fn update_category(&self, category: Category) -> PersistenceResult<Category> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let category_id = category.id();

        let result = conn
            .interact(move |conn| {
                diesel::update(schema::categories::table)
                    .filter(schema::categories::id.eq(category_id))
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
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        match result {
            None => Err(PersistenceError::Unexpected(
                "Category not updated".to_string(),
            )),
            Some(updated) => Ok(updated.try_into()?),
        }
    }

    async fn delete_category_by_id(&self, category_id: Uuid) -> PersistenceResult<()> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let affected_rows = conn
            .interact(move |conn| {
                diesel::delete(schema::categories::table)
                    .filter(schema::categories::id.eq(category_id))
                    .execute(conn)
            })
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        match affected_rows > 0 {
            true => Ok(()),
            false => Err(PersistenceError::NotFound("Category not found".to_string())),
        }
    }
}
