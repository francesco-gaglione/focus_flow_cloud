use crate::clients::{http_client::ApiError, task_http_client::create_subtask};

#[derive(Debug, thiserror::Error)]
pub enum CreateSubtaskError {
    #[error("Api error: {0}")]
    ApiError(#[from] ApiError),
}

pub async fn create_subtask_uc(
    task_id: String,
    title: String,
    description: Option<String>,
) -> Result<String, CreateSubtaskError> {
    let id = create_subtask(&task_id, title, description).await?;
    Ok(id)
}
