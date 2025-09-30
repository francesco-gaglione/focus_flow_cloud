use crate::db_models::db_category::{DbCategory, NewDbCategory, UpdateCategory};
use crate::repository::repository_error::RepositoryError;
use crate::schema::categories;
use chrono::Utc;
use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::DatabaseError;
use log::{debug, info, warn};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Clone)]
pub struct CategoryRepository {
    db_pool: Pool,
}

impl Debug for CategoryRepository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CategoryRepository")
            .field("pool_status", &"Connected")
            .field("pool_size", &self.db_pool.status().size)
            .field("available_connections", &self.db_pool.status().available)
            .finish()
    }
}

impl CategoryRepository {
    /// Creates a new CategoryRepository instance
    pub fn new(db_pool: Pool) -> Self {
        Self { db_pool }
    }

    /// Creates a new category in the database
    pub async fn create(&self, new_category: NewDbCategory) -> Result<DbCategory, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                diesel::insert_into(categories::table)
                    .values(&new_category)
                    .returning(DbCategory::as_returning())
                    .get_result(conn)
            })
            .await?
            .map_err(|e| {
                // Converti unique violation in un errore specifico
                match e {
                    DatabaseError(DatabaseErrorKind::UniqueViolation, info) => {
                        let message = info.message().to_string();
                        warn!("Unique constraint violation: {}", message);
                        RepositoryError::UniqueViolation(message)
                    }
                    other => RepositoryError::DieselError(other),
                }
            })?;

        info!("Created category with id: {}", result.id);
        Ok(result)
    }

    /// Retrieves a category by its ID
    pub async fn find_by_id(
        &self,
        category_id: Uuid,
    ) -> Result<Option<DbCategory>, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                categories::table
                    .filter(categories::id.eq(category_id))
                    .filter(categories::deleted_at.is_null())
                    .select(DbCategory::as_select())
                    .first(conn)
                    .optional()
            })
            .await??;

        if result.is_some() {
            debug!("Found category with id: {}", category_id);
        } else {
            debug!("No category found with id: {}", category_id);
        }

        Ok(result)
    }

    /// Retrieves a category by its name
    pub async fn find_by_name(&self, name: &str) -> Result<Option<DbCategory>, RepositoryError> {
        let name = name.to_string();
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                categories::table
                    .filter(categories::name.eq(name))
                    .filter(categories::deleted_at.is_null())
                    .select(DbCategory::as_select())
                    .first(conn)
                    .optional()
            })
            .await??;

        Ok(result)
    }

    /// Retrieves all non-deleted categories
    pub async fn find_all(&self) -> Result<Vec<DbCategory>, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                categories::table
                    .filter(categories::deleted_at.is_null())
                    .select(DbCategory::as_select())
                    .order(categories::created_at.desc())
                    .load(conn)
            })
            .await??;

        debug!("Retrieved {} categories", result.len());
        Ok(result)
    }

    /// Retrieves categories with pagination
    pub async fn find_paginated(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<DbCategory>, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                categories::table
                    .filter(categories::deleted_at.is_null())
                    .select(DbCategory::as_select())
                    .order(categories::created_at.desc())
                    .limit(limit)
                    .offset(offset)
                    .load(conn)
            })
            .await??;

        debug!(
            "Retrieved {} categories (limit: {}, offset: {})",
            result.len(),
            limit,
            offset
        );
        Ok(result)
    }

    /// Updates an existing category
    pub async fn update(
        &self,
        category_id: Uuid,
        update_data: UpdateCategory,
    ) -> Result<Option<DbCategory>, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                diesel::update(categories::table)
                    .filter(categories::id.eq(category_id))
                    .filter(categories::deleted_at.is_null())
                    .set((
                        &update_data,
                        categories::updated_at.eq(Utc::now()), // Manual updated_at handling
                    ))
                    .returning(DbCategory::as_returning())
                    .get_result(conn)
                    .optional()
            })
            .await??;

        if result.is_some() {
            info!("Updated category with id: {}", category_id);
        } else {
            warn!(
                "Attempted to update non-existent category with id: {}",
                category_id
            );
        }

        Ok(result)
    }

    /// Soft deletes a category (sets deleted_at timestamp)
    pub async fn soft_delete(&self, category_id: Uuid) -> Result<bool, RepositoryError> {
        let deleted_at = Utc::now();
        let conn = self.db_pool.get().await?;

        let affected_rows = conn
            .interact(move |conn| {
                diesel::update(categories::table)
                    .filter(categories::id.eq(category_id))
                    .filter(categories::deleted_at.is_null())
                    .set((
                        categories::deleted_at.eq(Some(deleted_at)),
                        categories::updated_at.eq(Utc::now()),
                    ))
                    .execute(conn)
            })
            .await??;

        let was_deleted = affected_rows > 0;
        if was_deleted {
            info!("Soft deleted category with id: {}", category_id);
        } else {
            warn!(
                "Attempted to soft delete non-existent category with id: {}",
                category_id
            );
        }

        Ok(was_deleted)
    }

    /// Permanently deletes a category from the database
    pub async fn hard_delete(&self, category_id: Uuid) -> Result<bool, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let affected_rows = conn
            .interact(move |conn| {
                diesel::delete(categories::table)
                    .filter(categories::id.eq(category_id))
                    .execute(conn)
            })
            .await??;

        let was_deleted = affected_rows > 0;
        if was_deleted {
            warn!("Permanently deleted category with id: {}", category_id);
        } else {
            warn!(
                "Attempted to permanently delete non-existent category with id: {}",
                category_id
            );
        }

        Ok(was_deleted)
    }

    /// Restores a soft-deleted category
    pub async fn restore(&self, category_id: Uuid) -> Result<bool, RepositoryError> {
        let updated_at = Utc::now();
        let conn = self.db_pool.get().await?;

        let affected_rows = conn
            .interact(move |conn| {
                diesel::update(categories::table)
                    .filter(categories::id.eq(category_id))
                    .filter(categories::deleted_at.is_not_null())
                    .set((
                        categories::deleted_at.eq(None::<chrono::DateTime<Utc>>),
                        categories::updated_at.eq(updated_at),
                    ))
                    .execute(conn)
            })
            .await??;

        let was_restored = affected_rows > 0;
        if was_restored {
            info!("Restored category with id: {}", category_id);
        } else {
            warn!(
                "Attempted to restore non-deleted category with id: {}",
                category_id
            );
        }

        Ok(was_restored)
    }

    /// Checks if a category exists by name
    pub async fn exists_by_name(&self, name: &str) -> Result<bool, RepositoryError> {
        let name = name.to_string();
        let conn = self.db_pool.get().await?;

        let exists = conn
            .interact(move |conn| {
                diesel::select(diesel::dsl::exists(
                    categories::table
                        .filter(categories::name.eq(name))
                        .filter(categories::deleted_at.is_null()),
                ))
                .get_result(conn)
            })
            .await??;

        Ok(exists)
    }

    /// Counts total number of non-deleted categories
    pub async fn count(&self) -> Result<i64, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let count = conn
            .interact(move |conn| {
                categories::table
                    .filter(categories::deleted_at.is_null())
                    .count()
                    .get_result(conn)
            })
            .await??;

        debug!("Total categories count: {}", count);
        Ok(count)
    }

    /// Searches categories by name (partial match)
    pub async fn search_by_name(
        &self,
        search_term: &str,
    ) -> Result<Vec<DbCategory>, RepositoryError> {
        let search_pattern = format!("%{}%", search_term);
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                categories::table
                    .filter(categories::name.ilike(search_pattern))
                    .filter(categories::deleted_at.is_null())
                    .select(DbCategory::as_select())
                    .order(categories::name.asc())
                    .load(conn)
            })
            .await??;

        debug!("Found {} categories matching search term", result.len());
        Ok(result)
    }
}
