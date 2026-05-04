use crate::clients::{http_client::ApiError, task_http_client::delete_task};

#[derive(Debug, thiserror::Error)]
pub enum DeleteTaskError {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error("Api error: {0}")]
    ApiError(#[from] ApiError),
}

pub type DeleteTaskResult<T> = Result<T, DeleteTaskError>;

pub async fn delete_task_uc(task_id: String) -> DeleteTaskResult<()> {
    delete_task(&task_id).await?;
    Ok(())
}
