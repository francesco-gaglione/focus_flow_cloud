use crate::http::app_state::AppState;
use crate::http::dto::validators::validate_uuid::validate_uuid;
use crate::http_error::{HttpError, HttpResult};
use crate::openapi::TASK_TAG;
use application::use_cases::task::update_task::{UpdateTaskCommand, UpdateTaskError};
use axum::extract::{Path, State};
use axum::Json;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

impl From<UpdateTaskError> for HttpError {
    fn from(value: UpdateTaskError) -> Self {
        match value {
            UpdateTaskError::PersistenceError(e) => HttpError::GenericError(e.to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateTaskPathDto {
    #[validate(custom(function = "validate_uuid"))]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTaskDto {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Title must be between 1 and 255 characters"
    ))]
    pub title: Option<String>,

    #[validate(length(max = 255, message = "Description must not exceed 255 characters"))]
    pub description: Option<String>,

    pub due_date: Option<i64>, // timestamp in seconds
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTaskResponseDto {
    pub success: bool,
}

#[utoipa::path(
    put,
    path = "/api/task/{id}",
    tag = TASK_TAG,
    summary = "Update a task",
    params(
        ("id" = String, Path, description = "Task ID to update")
    ),
    request_body = UpdateTaskDto,
    responses(
        (status = 200, description = "Task updated successfully", body = UpdateTaskResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Task not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn update_task_api(
    State(state): State<AppState>,
    Path(path): Path<UpdateTaskPathDto>,
    Json(payload): Json<UpdateTaskDto>,
) -> HttpResult<Json<UpdateTaskResponseDto>> {
    path.validate()
        .map_err(|e| HttpError::BadRequest(e.to_string()))?;

    payload
        .validate()
        .map_err(|e| HttpError::BadRequest(e.to_string()))?;

    let task_id = Uuid::parse_str(&path.id)
        .map_err(|_| HttpError::BadRequest("Task id malformed".to_string()))?;

    let due_date = payload
        .due_date
        .map(|s| {
            DateTime::from_timestamp(s, 0)
                .ok_or_else(|| HttpError::BadRequest("Invalid due date".to_string()))
        })
        .transpose()?;

    let command = UpdateTaskCommand {
        id: task_id,
        title: payload.title,
        description: payload.description,
        due_date,
        priority: None,
    };

    state.update_task_uc.execute(command).await?;

    Ok(Json(UpdateTaskResponseDto { success: true }))
}
