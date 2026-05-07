use chrono::{DateTime, Utc};

use crate::clients::{http_client::ApiError, task_http_client::update_task};

#[derive(Debug, thiserror::Error)]
pub enum UpdateTaskDueDateError {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error("Api error: {0}")]
    ApiError(#[from] ApiError),
}

pub type UpdateTaskDueDateResult<T> = Result<T, UpdateTaskDueDateError>;

pub async fn update_task_due_date_uc(
    task_id: &str,
    due_date: Option<DateTime<Utc>>,
) -> UpdateTaskDueDateResult<()> {
    update_task(task_id, None, None, due_date, None, None).await?;
    Ok(())
}
