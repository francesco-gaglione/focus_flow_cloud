use crate::application::app_error::AppError;
use deadpool_diesel::postgres::Pool;
use diesel::result::Error;

mod category_persistence_impl;
pub mod db_models;
pub mod focus_session_persistence_impl;
mod task_persistence_impl;
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

impl From<Error> for AppError {
    fn from(value: Error) -> Self {
        AppError::Database(value.to_string())
    }
}
