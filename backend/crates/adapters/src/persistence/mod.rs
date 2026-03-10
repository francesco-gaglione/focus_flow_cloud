use std::sync::Arc;

use deadpool_diesel::postgres::Pool;
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
}
