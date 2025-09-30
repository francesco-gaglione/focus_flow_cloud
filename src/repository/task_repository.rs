use crate::db_models::db_task::{DbTask, NewDbTask, UpdateDbTask};
use crate::repository::repository_error::RepositoryError;
use crate::schema::tasks;
use chrono::{NaiveDate, Utc};
use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::DatabaseError;
use log::{debug, info, warn};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Clone)]
pub struct TaskRepository {
    db_pool: Pool,
}

impl Debug for TaskRepository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TaskRepository")
            .field("pool_status", &"Connected")
            .field("pool_size", &self.db_pool.status().size)
            .field("available_connections", &self.db_pool.status().available)
            .finish()
    }
}

impl TaskRepository {
    /// Creates a new TaskRepository instance
    pub fn new(db_pool: Pool) -> Self {
        Self { db_pool }
    }

    /// Creates a new task in the database
    pub async fn create(&self, new_task: NewDbTask) -> Result<DbTask, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                diesel::insert_into(tasks::table)
                    .values(&new_task)
                    .returning(DbTask::as_returning())
                    .get_result(conn)
            })
            .await?
            .map_err(|e| match e {
                DatabaseError(DatabaseErrorKind::UniqueViolation, info) => {
                    let message = info.message().to_string();
                    warn!("Unique constraint violation: {}", message);
                    RepositoryError::UniqueViolation(message)
                }
                DatabaseError(DatabaseErrorKind::ForeignKeyViolation, info) => {
                    let message = info.message().to_string();
                    warn!("Foreign key constraint violation: {}", message);
                    RepositoryError::ForeignKeyViolation(message)
                }
                other => RepositoryError::DieselError(other),
            })?;

        info!("Created task with id: {}", result.id);
        Ok(result)
    }

    /// Retrieves a task by its ID
    pub async fn find_by_id(&self, task_id: Uuid) -> Result<Option<DbTask>, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                tasks::table
                    .filter(tasks::id.eq(task_id))
                    .filter(tasks::deleted_at.is_null())
                    .select(DbTask::as_select())
                    .first(conn)
                    .optional()
            })
            .await??;

        if result.is_some() {
            debug!("Found task with id: {}", task_id);
        } else {
            debug!("No task found with id: {}", task_id);
        }

        Ok(result)
    }

    /// Retrieves a task by its name
    pub async fn find_by_name(&self, name: &str) -> Result<Option<DbTask>, RepositoryError> {
        let name = name.to_string();
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                tasks::table
                    .filter(tasks::name.eq(name))
                    .filter(tasks::deleted_at.is_null())
                    .select(DbTask::as_select())
                    .first(conn)
                    .optional()
            })
            .await??;

        Ok(result)
    }

    /// Retrieves all non-deleted tasks
    pub async fn find_all(&self) -> Result<Vec<DbTask>, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                tasks::table
                    .filter(tasks::deleted_at.is_null())
                    .select(DbTask::as_select())
                    .order(tasks::created_at.desc())
                    .load(conn)
            })
            .await??;

        debug!("Retrieved {} tasks", result.len());
        Ok(result)
    }

    /// Retrieves all tasks for a specific category
    pub async fn find_by_category(
        &self,
        category_id: Uuid,
    ) -> Result<Vec<DbTask>, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                tasks::table
                    .filter(tasks::category_id.eq(category_id))
                    .filter(tasks::deleted_at.is_null())
                    .select(DbTask::as_select())
                    .order(tasks::created_at.desc())
                    .load(conn)
            })
            .await??;

        debug!(
            "Retrieved {} tasks for category {}",
            result.len(),
            category_id
        );
        Ok(result)
    }

    /// Retrieves all tasks for a specific template
    pub async fn find_by_template(
        &self,
        template_id: Uuid,
    ) -> Result<Vec<DbTask>, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                tasks::table
                    .filter(tasks::template_id.eq(template_id))
                    .filter(tasks::deleted_at.is_null())
                    .select(DbTask::as_select())
                    .order(tasks::created_at.desc())
                    .load(conn)
            })
            .await??;

        debug!(
            "Retrieved {} tasks for template {}",
            result.len(),
            template_id
        );
        Ok(result)
    }

    /// Retrieves tasks scheduled for a specific date
    pub async fn find_by_scheduled_date(
        &self,
        date: NaiveDate,
    ) -> Result<Vec<DbTask>, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                tasks::table
                    .filter(tasks::scheduled_date.eq(date))
                    .filter(tasks::deleted_at.is_null())
                    .select(DbTask::as_select())
                    .order(tasks::created_at.desc())
                    .load(conn)
            })
            .await??;

        debug!("Retrieved {} tasks for date {}", result.len(), date);
        Ok(result)
    }

    /// Retrieves all completed tasks
    pub async fn find_completed(&self) -> Result<Vec<DbTask>, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                tasks::table
                    .filter(tasks::completed_at.is_not_null())
                    .filter(tasks::deleted_at.is_null())
                    .select(DbTask::as_select())
                    .order(tasks::completed_at.desc())
                    .load(conn)
            })
            .await??;

        debug!("Retrieved {} completed tasks", result.len());
        Ok(result)
    }

    /// Retrieves all incomplete (pending) tasks
    pub async fn find_pending(&self) -> Result<Vec<DbTask>, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                tasks::table
                    .filter(tasks::completed_at.is_null())
                    .filter(tasks::deleted_at.is_null())
                    .select(DbTask::as_select())
                    .order(tasks::created_at.desc())
                    .load(conn)
            })
            .await??;

        debug!("Retrieved {} pending tasks", result.len());
        Ok(result)
    }

    /// Retrieves tasks with pagination
    pub async fn find_paginated(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<DbTask>, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                tasks::table
                    .filter(tasks::deleted_at.is_null())
                    .select(DbTask::as_select())
                    .order(tasks::created_at.desc())
                    .limit(limit)
                    .offset(offset)
                    .load(conn)
            })
            .await??;

        debug!(
            "Retrieved {} tasks (limit: {}, offset: {})",
            result.len(),
            limit,
            offset
        );
        Ok(result)
    }

    /// Updates an existing task
    pub async fn update(
        &self,
        task_id: Uuid,
        update_data: UpdateDbTask,
    ) -> Result<Option<DbTask>, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                diesel::update(tasks::table)
                    .filter(tasks::id.eq(task_id))
                    .filter(tasks::deleted_at.is_null())
                    .set(&update_data)
                    .returning(DbTask::as_returning())
                    .get_result(conn)
                    .optional()
            })
            .await??;

        if result.is_some() {
            info!("Updated task with id: {}", task_id);
        } else {
            warn!("Attempted to update non-existent task with id: {}", task_id);
        }

        Ok(result)
    }

    /// Marks a task as completed
    pub async fn mark_completed(&self, task_id: Uuid) -> Result<Option<DbTask>, RepositoryError> {
        let completed_at = Utc::now();
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                diesel::update(tasks::table)
                    .filter(tasks::id.eq(task_id))
                    .filter(tasks::deleted_at.is_null())
                    .filter(tasks::completed_at.is_null())
                    .set(tasks::completed_at.eq(Some(completed_at)))
                    .returning(DbTask::as_returning())
                    .get_result(conn)
                    .optional()
            })
            .await??;

        if result.is_some() {
            info!("Marked task {} as completed", task_id);
        } else {
            warn!(
                "Attempted to complete non-existent or already completed task: {}",
                task_id
            );
        }

        Ok(result)
    }

    /// Marks a task as incomplete (uncompletes it)
    pub async fn mark_incomplete(&self, task_id: Uuid) -> Result<Option<DbTask>, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                diesel::update(tasks::table)
                    .filter(tasks::id.eq(task_id))
                    .filter(tasks::deleted_at.is_null())
                    .filter(tasks::completed_at.is_not_null())
                    .set(tasks::completed_at.eq(None::<chrono::DateTime<Utc>>))
                    .returning(DbTask::as_returning())
                    .get_result(conn)
                    .optional()
            })
            .await??;

        if result.is_some() {
            info!("Marked task {} as incomplete", task_id);
        } else {
            warn!(
                "Attempted to uncomplete non-existent or already incomplete task: {}",
                task_id
            );
        }

        Ok(result)
    }

    /// Soft deletes a task (sets deleted_at timestamp)
    pub async fn soft_delete(&self, task_id: Uuid) -> Result<bool, RepositoryError> {
        let deleted_at = Utc::now();
        let conn = self.db_pool.get().await?;

        let affected_rows = conn
            .interact(move |conn| {
                diesel::update(tasks::table)
                    .filter(tasks::id.eq(task_id))
                    .filter(tasks::deleted_at.is_null())
                    .set(tasks::deleted_at.eq(Some(deleted_at)))
                    .execute(conn)
            })
            .await??;

        let was_deleted = affected_rows > 0;
        if was_deleted {
            info!("Soft deleted task with id: {}", task_id);
        } else {
            warn!(
                "Attempted to soft delete non-existent task with id: {}",
                task_id
            );
        }

        Ok(was_deleted)
    }

    /// Permanently deletes a task from the database
    pub async fn hard_delete(&self, task_id: Uuid) -> Result<bool, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let affected_rows = conn
            .interact(move |conn| {
                diesel::delete(tasks::table)
                    .filter(tasks::id.eq(task_id))
                    .execute(conn)
            })
            .await??;

        let was_deleted = affected_rows > 0;
        if was_deleted {
            warn!("Permanently deleted task with id: {}", task_id);
        } else {
            warn!(
                "Attempted to permanently delete non-existent task with id: {}",
                task_id
            );
        }

        Ok(was_deleted)
    }

    /// Restores a soft-deleted task
    pub async fn restore(&self, task_id: Uuid) -> Result<bool, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let affected_rows = conn
            .interact(move |conn| {
                diesel::update(tasks::table)
                    .filter(tasks::id.eq(task_id))
                    .filter(tasks::deleted_at.is_not_null())
                    .set(tasks::deleted_at.eq(None::<chrono::DateTime<Utc>>))
                    .execute(conn)
            })
            .await??;

        let was_restored = affected_rows > 0;
        if was_restored {
            info!("Restored task with id: {}", task_id);
        } else {
            warn!("Attempted to restore non-deleted task with id: {}", task_id);
        }

        Ok(was_restored)
    }

    /// Checks if a task exists by name
    pub async fn exists_by_name(&self, name: &str) -> Result<bool, RepositoryError> {
        let name = name.to_string();
        let conn = self.db_pool.get().await?;

        let exists = conn
            .interact(move |conn| {
                diesel::select(diesel::dsl::exists(
                    tasks::table
                        .filter(tasks::name.eq(name))
                        .filter(tasks::deleted_at.is_null()),
                ))
                .get_result(conn)
            })
            .await??;

        Ok(exists)
    }

    /// Counts total number of non-deleted tasks
    pub async fn count(&self) -> Result<i64, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let count = conn
            .interact(move |conn| {
                tasks::table
                    .filter(tasks::deleted_at.is_null())
                    .count()
                    .get_result(conn)
            })
            .await??;

        debug!("Total tasks count: {}", count);
        Ok(count)
    }

    /// Counts completed tasks
    pub async fn count_completed(&self) -> Result<i64, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let count = conn
            .interact(move |conn| {
                tasks::table
                    .filter(tasks::completed_at.is_not_null())
                    .filter(tasks::deleted_at.is_null())
                    .count()
                    .get_result(conn)
            })
            .await??;

        debug!("Completed tasks count: {}", count);
        Ok(count)
    }

    /// Counts pending tasks
    pub async fn count_pending(&self) -> Result<i64, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let count = conn
            .interact(move |conn| {
                tasks::table
                    .filter(tasks::completed_at.is_null())
                    .filter(tasks::deleted_at.is_null())
                    .count()
                    .get_result(conn)
            })
            .await??;

        debug!("Pending tasks count: {}", count);
        Ok(count)
    }

    /// Searches tasks by name (partial match)
    pub async fn search_by_name(&self, search_term: &str) -> Result<Vec<DbTask>, RepositoryError> {
        let search_pattern = format!("%{}%", search_term);
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                tasks::table
                    .filter(tasks::name.ilike(search_pattern))
                    .filter(tasks::deleted_at.is_null())
                    .select(DbTask::as_select())
                    .order(tasks::name.asc())
                    .load(conn)
            })
            .await??;

        debug!("Found {} tasks matching search term", result.len());
        Ok(result)
    }

    /// Searches tasks by description (partial match)
    pub async fn search_by_description(
        &self,
        search_term: &str,
    ) -> Result<Vec<DbTask>, RepositoryError> {
        let search_pattern = format!("%{}%", search_term);
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                tasks::table
                    .filter(tasks::description.ilike(search_pattern))
                    .filter(tasks::deleted_at.is_null())
                    .select(DbTask::as_select())
                    .order(tasks::created_at.desc())
                    .load(conn)
            })
            .await??;

        debug!("Found {} tasks matching description search", result.len());
        Ok(result)
    }
}
