use std::str::FromStr;

use crate::http::app_state::AppState;
use crate::http::dto::validators::validate_uuid::validate_uuid;
use crate::http::model::session_model::UserSession;
use crate::http_error::{HttpError, HttpResult};
use crate::openapi::TASK_TAG;
use application::use_cases::task::complete_task::{CompleteTaskCommand, CompleteTaskError};
use axum::{extract::State, Extension, Json};
use serde::{Deserialize, Serialize};
use tracing::debug;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CompleteTaskDto {
    #[validate(custom(function = "validate_uuid"))]
    pub task_id: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CompleteTaskResponseDto {
    pub id: String,
}

impl From<CompleteTaskError> for HttpError {
    fn from(value: CompleteTaskError) -> Self {
        match value {
            CompleteTaskError::TaskNotFound(task_id) => HttpError::NotFound(task_id.to_string()),
            CompleteTaskError::PersistenceError(persistence_error) => {
                HttpError::GenericError(persistence_error.to_string())
            }
            CompleteTaskError::Unauthorized => HttpError::Unauthorized(
                "Current user is not authorized to complete this task".to_string(),
            ),
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/task/complete",
    tag = TASK_TAG,
    summary = "Complete a task",
    request_body = CompleteTaskDto,
    responses(
        (status = 201, description = "Task completed successfully", body = CompleteTaskResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 409, description = "Task already exists"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn complete_task_api(
    State(state): State<AppState>,
    Extension(user): Extension<UserSession>,
    Json(payload): Json<CompleteTaskDto>,
) -> HttpResult<Json<CompleteTaskResponseDto>> {
    payload
        .validate()
        .map_err(|e| HttpError::BadRequest(e.to_string()))?;

    let task_id = Uuid::from_str(payload.task_id.as_str())
        .map_err(|e| HttpError::BadRequest(e.to_string()))?;

    let command = CompleteTaskCommand {
        id: task_id,
        user_id: user.user_id,
    };

    debug!("Executing complete task command: {:?}", command);

    state.complete_task_usecase.execute(command).await?;

    Ok(Json(CompleteTaskResponseDto {
        id: task_id.to_string(),
    }))
}
