use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    // CategoryService errors
    #[error("Category already exists")]
    CategoryAlreadyExists,
    #[error("Category not found")]
    CategoryNotFound,

    // Common service errors
    #[error("Repository error: {0}")]
    RepositoryError(#[from] crate::repository::repository_error::RepositoryError),

    #[error("Generic error")]
    GenericError,
}
