use deadpool_diesel::postgres::Pool;

mod category_persistence_impl;
pub mod db_models;
pub mod focus_session_persistence_impl;
mod task_persistence_impl;
pub mod user_persistence_impl;
pub mod user_setting_persistence_impl;

#[derive(Clone)]
pub struct PostgresPersistence {
    pool: Pool,
}

impl PostgresPersistence {
    pub fn new(pool: Pool) -> Self {
        PostgresPersistence { pool }
    }
}
