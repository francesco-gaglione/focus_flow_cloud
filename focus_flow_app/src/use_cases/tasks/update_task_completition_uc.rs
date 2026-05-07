use crate::clients::{http_client::ApiError, task_http_client::update_task};

#[derive(Debug, thiserror::Error)]
pub enum CompleteTaskError {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error("Api error: {0}")]
    ApiError(#[from] ApiError),
}

pub type CompleteTaskResult<T> = Result<T, CompleteTaskError>;

pub async fn update_task_completition_uc(
    task_id: &str,
    completed: Option<bool>,
) -> CompleteTaskResult<()> {
    update_task(task_id, None, None, None, completed, None).await?;
    Ok(())
}
