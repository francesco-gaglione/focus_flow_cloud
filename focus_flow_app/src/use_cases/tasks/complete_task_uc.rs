use crate::clients::{http_client::ApiError, task_http_client::complete_task};

#[derive(Debug, thiserror::Error)]
pub enum CompleteTaskError {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error("Api error: {0}")]
    ApiError(#[from] ApiError),
}

pub type CompleteTaskResult<T> = Result<T, CompleteTaskError>;

pub async fn complete_task_uc(task_id: &str) -> CompleteTaskResult<()> {
    complete_task(task_id).await?;
    Ok(())
}
