use crate::clients::{http_client::ApiError, task_http_client::complete_sub_task};

#[derive(Debug, thiserror::Error)]
pub enum CompleteSubTaskError {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error("Api error: {0}")]
    ApiError(#[from] ApiError),
}

pub type CompleteSubTaskResult<T> = Result<T, CompleteSubTaskError>;

pub async fn complete_task_uc(task_id: String, subtask_id: String) -> CompleteSubTaskResult<()> {
    complete_sub_task(&task_id, &subtask_id).await?;
    Ok(())
}
