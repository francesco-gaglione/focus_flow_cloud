use deadpool_diesel::postgres::Pool;

pub mod db_models;
pub mod persistence_impl;
pub mod schema;

#[derive(Clone)]
pub struct PostgresPersistence {
    pub pool: Pool,
}

impl PostgresPersistence {
    pub fn new(pool: Pool) -> Self {
        PostgresPersistence { pool }
    }
}
