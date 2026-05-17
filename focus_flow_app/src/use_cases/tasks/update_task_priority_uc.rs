use shared::task::TaskPriority;

use crate::clients::{http_client::ApiError, task_http_client::update_task};

#[derive(Debug, thiserror::Error)]
pub enum UpdateTaskPriorityError {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error("Api error: {0}")]
    ApiError(#[from] ApiError),
}

pub type UpdateTaskPriorityResult<T> = Result<T, UpdateTaskPriorityError>;

pub async fn update_task_priority_uc(
    task_id: &str,
    priority: Option<TaskPriority>,
) -> UpdateTaskPriorityResult<()> {
    update_task(task_id, None, None, None, None, priority).await?;
    Ok(())
}
