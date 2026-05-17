use crate::clients::{http_client::ApiError, task_http_client::udpate_subtask};

#[derive(Debug, thiserror::Error)]
pub enum CompleteSubTaskError {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error("Api error: {0}")]
    ApiError(#[from] ApiError),
}

pub type CompleteSubTaskResult<T> = Result<T, CompleteSubTaskError>;

pub async fn update_subtask_completition_uc(
    task_id: String,
    subtask_id: String,
    completed: Option<bool>,
) -> CompleteSubTaskResult<()> {
    udpate_subtask(&task_id, &subtask_id, completed).await?;
    Ok(())
}
