use crate::application::app_error::AppError;
use deadpool_diesel::postgres::Pool;
use diesel::result::Error;

pub mod db_models;
mod category_persistence_impl;
mod task_persistence_impl;

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
