use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Category already exists")]
    CategoryAlreadyExists,
    #[error("Category not found")]
    CategoryNotFound,
    #[error("Repository error: {0}")]
    RepositoryError(#[from] crate::repository::repository_error::RepositoryError),
}
