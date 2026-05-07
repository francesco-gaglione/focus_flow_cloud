use shared::task::TaskScheduleDto;

use crate::clients::{http_client::ApiError, task_http_client::update_task};

#[derive(Debug, thiserror::Error)]
pub enum UpdateTaskScheduleError {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error("Api error: {0}")]
    ApiError(#[from] ApiError),
}

pub type UpdateTaskScheduleResult<T> = Result<T, UpdateTaskScheduleError>;

/// Pass `None` to leave schedule unchanged, `Some(TaskScheduleDto::Unscheduled)` to clear it.
pub async fn update_task_due_date_uc(
    task_id: &str,
    schedule: Option<TaskScheduleDto>,
) -> UpdateTaskScheduleResult<()> {
    update_task(task_id, None, None, schedule, None, None).await?;
    Ok(())
}
