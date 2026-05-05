use std::str::FromStr;

use crate::http::app_state::AppState;
use crate::http::model::session_model::UserSession;
use crate::http_error::{HttpError, HttpResult};
use crate::openapi::TASK_TAG;
use application::use_cases::task::complete_subtask::{
    CompleteSubTaskCommand, CompleteSubTaskError,
};
use axum::{extract::State, Extension, Json};
use shared::task::{CompleteSubTaskDto, CompleteSubTaskResponseDto};
use tracing::{debug, error};
use uuid::Uuid;
use validator::Validate;

impl From<CompleteSubTaskError> for HttpError {
    fn from(value: CompleteSubTaskError) -> Self {
        match value {
            CompleteSubTaskError::TaskNotFound(task_id) => HttpError::NotFound(task_id.to_string()),
            CompleteSubTaskError::SubTaskNotFound(task_id) => {
                HttpError::NotFound(task_id.to_string())
            }
            CompleteSubTaskError::PersistenceError(persistence_error) => {
                HttpError::GenericError(persistence_error.to_string())
            }
            CompleteSubTaskError::Unauthorized => HttpError::Unauthorized(
                "Current user is not authorized to complete this task".to_string(),
            ),
            CompleteSubTaskError::UncompletedSubTasks => {
                HttpError::BadRequest("Sub-tasks must be completed first".to_string())
            }
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/task/subtask/complete",
    tag = TASK_TAG,
    summary = "Complete a subtask",
    request_body = CompleteSubTaskDto,
    responses(
        (status = 201, description = "Task completed successfully", body = CompleteSubTaskResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 409, description = "Task already exists"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn complete_sub_task_api(
    State(state): State<AppState>,
    Extension(user): Extension<UserSession>,
    Json(payload): Json<CompleteSubTaskDto>,
) -> HttpResult<Json<CompleteSubTaskResponseDto>> {
    payload.validate().map_err(|e| {
        error!("Validation error: {}", e);
        HttpError::BadRequest(e.to_string())
    })?;

    let task_id = Uuid::from_str(payload.task_id.as_str())
        .map_err(|e| HttpError::BadRequest(e.to_string()))?;

    let sub_task_id = Uuid::from_str(&payload.subtask_id.as_str())
        .map_err(|e| HttpError::BadRequest(e.to_string()))?;

    let command = CompleteSubTaskCommand {
        user_id: user.user_id,
        task_id,
        sub_task_id,
    };

    debug!("Executing complete subtask command: {:?}", command);

    state.complete_sub_task_uc.execute(command).await?;

    Ok(Json(CompleteSubTaskResponseDto {
        id: sub_task_id.to_string(),
    }))
}
