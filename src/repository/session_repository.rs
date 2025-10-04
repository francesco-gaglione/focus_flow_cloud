use crate::db_models::db_focus_session::{DbFocusSession, NewDbFocusSession, UpdateDbFocusSession};
use crate::repository::repository_error::RepositoryError;
use crate::schema::focus_session;
use chrono::Utc;
use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
use log::{debug, info, warn};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Clone)]
pub struct FocusSessionRepository {
    db_pool: Pool,
}

impl Debug for FocusSessionRepository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FocusSessionRepository")
            .field("pool_status", &"Connected")
            .field("pool_size", &self.db_pool.status().size)
            .field("available_connections", &self.db_pool.status().available)
            .finish()
    }
}

impl FocusSessionRepository {
    /// Creates a new FocusSessionRepository instance
    pub fn new(db_pool: Pool) -> Self {
        Self { db_pool }
    }

    /// Creates a new focus session in the database
    pub async fn create(
        &self,
        new_session: NewDbFocusSession,
    ) -> Result<DbFocusSession, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                diesel::insert_into(focus_session::table)
                    .values(&new_session)
                    .returning(DbFocusSession::as_returning())
                    .get_result(conn)
            })
            .await??;

        info!("Created focus session with id: {}", result.id);
        Ok(result)
    }

    /// Retrieves a focus session by its ID
    pub async fn find_by_id(
        &self,
        session_id: Uuid,
    ) -> Result<Option<DbFocusSession>, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                focus_session::table
                    .filter(focus_session::id.eq(session_id))
                    .select(DbFocusSession::as_select())
                    .first(conn)
                    .optional()
            })
            .await??;

        if result.is_some() {
            debug!("Found focus session with id: {}", session_id);
        } else {
            debug!("No focus session found with id: {}", session_id);
        }

        Ok(result)
    }

    /// Retrieves all focus sessions ordered by start time (descending)
    pub async fn find_all(&self) -> Result<Vec<DbFocusSession>, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                focus_session::table
                    .select(DbFocusSession::as_select())
                    .order(focus_session::started_at.desc())
                    .load(conn)
            })
            .await??;

        debug!("Retrieved {} focus sessions", result.len());
        Ok(result)
    }

    /// Retrieves focus sessions by category ID
    pub async fn find_by_category(
        &self,
        category_id: Uuid,
    ) -> Result<Vec<DbFocusSession>, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                focus_session::table
                    .filter(focus_session::category_id.eq(category_id))
                    .select(DbFocusSession::as_select())
                    .order(focus_session::started_at.desc())
                    .load(conn)
            })
            .await??;

        debug!(
            "Retrieved {} focus sessions for category: {}",
            result.len(),
            category_id
        );
        Ok(result)
    }

    /// Retrieves focus sessions by task ID
    pub async fn find_by_task(
        &self,
        task_id: Uuid,
    ) -> Result<Vec<DbFocusSession>, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                focus_session::table
                    .filter(focus_session::task_id.eq(task_id))
                    .select(DbFocusSession::as_select())
                    .order(focus_session::started_at.desc())
                    .load(conn)
            })
            .await??;

        debug!(
            "Retrieved {} focus sessions for task: {}",
            result.len(),
            task_id
        );
        Ok(result)
    }

    /// Retrieves active (not ended) focus sessions
    pub async fn find_active(&self) -> Result<Vec<DbFocusSession>, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                focus_session::table
                    .filter(focus_session::ended_at.is_null())
                    .select(DbFocusSession::as_select())
                    .order(focus_session::started_at.desc())
                    .load(conn)
            })
            .await??;

        debug!("Retrieved {} active focus sessions", result.len());
        Ok(result)
    }

    /// Retrieves completed (ended) focus sessions
    pub async fn find_completed(&self) -> Result<Vec<DbFocusSession>, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                focus_session::table
                    .filter(focus_session::ended_at.is_not_null())
                    .select(DbFocusSession::as_select())
                    .order(focus_session::started_at.desc())
                    .load(conn)
            })
            .await??;

        debug!("Retrieved {} completed focus sessions", result.len());
        Ok(result)
    }

    /// Retrieves focus sessions by session type
    pub async fn find_by_type(
        &self,
        session_type: &str,
    ) -> Result<Vec<DbFocusSession>, RepositoryError> {
        let session_type = session_type.to_string();
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                focus_session::table
                    .filter(focus_session::session_type.eq(session_type))
                    .select(DbFocusSession::as_select())
                    .order(focus_session::started_at.desc())
                    .load(conn)
            })
            .await??;

        debug!("Retrieved {} focus sessions of type", result.len());
        Ok(result)
    }

    /// Retrieves focus sessions with pagination
    pub async fn find_paginated(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<DbFocusSession>, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                focus_session::table
                    .select(DbFocusSession::as_select())
                    .order(focus_session::started_at.desc())
                    .limit(limit)
                    .offset(offset)
                    .load(conn)
            })
            .await??;

        debug!(
            "Retrieved {} focus sessions (limit: {}, offset: {})",
            result.len(),
            limit,
            offset
        );
        Ok(result)
    }

    /// Retrieves focus sessions within a date range
    pub async fn find_by_date_range(
        &self,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<DbFocusSession>, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                focus_session::table
                    .filter(focus_session::started_at.ge(start_date))
                    .filter(focus_session::started_at.le(end_date))
                    .select(DbFocusSession::as_select())
                    .order(focus_session::started_at.desc())
                    .load(conn)
            })
            .await??;

        debug!(
            "Retrieved {} focus sessions between {} and {}",
            result.len(),
            start_date,
            end_date
        );
        Ok(result)
    }

    /// Updates an existing focus session
    pub async fn update(
        &self,
        session_id: Uuid,
        update_data: UpdateDbFocusSession,
    ) -> Result<Option<DbFocusSession>, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                diesel::update(focus_session::table)
                    .filter(focus_session::id.eq(session_id))
                    .set(&update_data)
                    .returning(DbFocusSession::as_returning())
                    .get_result(conn)
                    .optional()
            })
            .await??;

        if result.is_some() {
            info!("Updated focus session with id: {}", session_id);
        } else {
            warn!(
                "Attempted to update non-existent focus session with id: {}",
                session_id
            );
        }

        Ok(result)
    }

    /// Ends a focus session by setting the ended_at timestamp and actual duration
    pub async fn end_session(
        &self,
        session_id: Uuid,
        actual_duration_minutes: i32,
        concentration_score: Option<i32>,
        notes: Option<String>,
    ) -> Result<Option<DbFocusSession>, RepositoryError> {
        let ended_at = Utc::now();
        let conn = self.db_pool.get().await?;

        let result = conn
            .interact(move |conn| {
                diesel::update(focus_session::table)
                    .filter(focus_session::id.eq(session_id))
                    .filter(focus_session::ended_at.is_null())
                    .set((
                        focus_session::ended_at.eq(Some(ended_at)),
                        focus_session::actual_duration_minutes.eq(Some(actual_duration_minutes)),
                        focus_session::concentration_score.eq(concentration_score),
                        focus_session::notes.eq(notes),
                    ))
                    .returning(DbFocusSession::as_returning())
                    .get_result(conn)
                    .optional()
            })
            .await??;

        if result.is_some() {
            info!("Ended focus session with id: {}", session_id);
        } else {
            warn!(
                "Attempted to end non-existent or already ended focus session with id: {}",
                session_id
            );
        }

        Ok(result)
    }

    /// Permanently deletes a focus session from the database
    pub async fn delete(&self, session_id: Uuid) -> Result<bool, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let affected_rows = conn
            .interact(move |conn| {
                diesel::delete(focus_session::table)
                    .filter(focus_session::id.eq(session_id))
                    .execute(conn)
            })
            .await??;

        let was_deleted = affected_rows > 0;
        if was_deleted {
            warn!("Permanently deleted focus session with id: {}", session_id);
        } else {
            warn!(
                "Attempted to delete non-existent focus session with id: {}",
                session_id
            );
        }

        Ok(was_deleted)
    }

    /// Counts total number of focus sessions
    pub async fn count(&self) -> Result<i64, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let count = conn
            .interact(move |conn| focus_session::table.count().get_result(conn))
            .await??;

        debug!("Total focus sessions count: {}", count);
        Ok(count)
    }

    /// Counts focus sessions by category
    pub async fn count_by_category(&self, category_id: Uuid) -> Result<i64, RepositoryError> {
        let conn = self.db_pool.get().await?;

        let count = conn
            .interact(move |conn| {
                focus_session::table
                    .filter(focus_session::category_id.eq(category_id))
                    .count()
                    .get_result(conn)
            })
            .await??;

        debug!(
            "Focus sessions count for category {}: {}",
            category_id, count
        );
        Ok(count)
    }
}
