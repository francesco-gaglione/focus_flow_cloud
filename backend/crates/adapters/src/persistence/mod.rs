use std::sync::Arc;

use application::repository_traits::persistence_error::{PersistenceError, PersistenceResult};
use deadpool_diesel::postgres::Pool;
use diesel::{pg::PgConnection, result::QueryResult, Connection};
use domain::entities::focus_session::{FocusSession, RunningSession};
use tokio::sync::RwLock;

pub mod db_models;
pub mod persistence_impl;
pub mod schema;

#[derive(Clone)]
pub struct PostgresPersistence {
    pub pool: Pool,
    pub running_session_cache: Arc<RwLock<Option<FocusSession<RunningSession>>>>,
}

impl PostgresPersistence {
    pub fn new(pool: Pool) -> Self {
        PostgresPersistence {
            pool,
            running_session_cache: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn with_transaction<F, T>(&self, f: F) -> PersistenceResult<T>
    where
        F: FnOnce(&mut PgConnection) -> QueryResult<T> + Send + 'static,
        T: Send + 'static,
    {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        conn.interact(move |conn| conn.transaction(f))
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))
    }
}
