use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Database connection error: {0}")]
    DatabaseConnection(String),

    #[error("Database interaction error: {0}")]
    DatabaseInteraction(String),

    #[error("Resource not found")]
    NotFound,

    #[error("Diesel error: {0}")]
    DieselError(#[from] diesel::result::Error),

    #[error("Pool error: {0}")]
    PoolError(#[from] deadpool_diesel::PoolError),

    #[error("Database interaction failed")]
    InteractError(#[from] deadpool_diesel::InteractError),
}
