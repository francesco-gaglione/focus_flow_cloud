use crate::shared::http::app_state::AppState;
use crate::shared::http::model::session_model::UserSession;
use crate::shared::http::validators::validate_uuid::validate_uuid;
use crate::http_error::{HttpError, HttpResult};
use crate::openapi::TASK_TAG;
use application::tasks::use_cases::task::update_subtask::{
    UpdateSubTaskCommand, UpdateSubTaskError,
};
use axum::extract::Path;
use axum::{extract::State, Extension, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct UpdateSubTaskDto {
    pub completed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct UpdateSubTaskResponseDto {
    pub id: String,
}
use std::str::FromStr;
use tracing::{debug, error};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

impl From<UpdateSubTaskError> for HttpError {
    fn from(value: UpdateSubTaskError) -> Self {
        match value {
            UpdateSubTaskError::PersistenceError(e) => HttpError::GenericError(e.to_string()),
            UpdateSubTaskError::TaskNotFound(uuid) => {
                HttpError::NotFound(format!("Task {} not found", uuid))
            }
            UpdateSubTaskError::SubTaskNotFound(uuid) => {
                HttpError::NotFound(format!("Subtask {} not found", uuid))
            }
            UpdateSubTaskError::Unauthorized => HttpError::Forbidden,
            UpdateSubTaskError::UncompletedSubTasks => {
                HttpError::BadRequest("All subtasks must be completed first".to_string())
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateSubTaskPathDto {
    #[validate(custom(function = "validate_uuid"))]
    pub id: String,

    #[validate(custom(function = "validate_uuid"))]
    pub subtask_id: String,
}

#[utoipa::path(
    patch,
    path = "/api/task/{id}/subtask/{subtask_id}",
    tag = TASK_TAG,
    summary = "Update a subtask",
    request_body = UpdateSubTaskDto,
    responses(
        (status = 201, description = "Subtask completed successfully", body = UpdateSubTaskResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn update_subtask_api(
    State(state): State<AppState>,
    Extension(user): Extension<UserSession>,
    Path(path): Path<UpdateSubTaskPathDto>,
    Json(payload): Json<UpdateSubTaskDto>,
) -> HttpResult<Json<UpdateSubTaskResponseDto>> {
    payload.validate().map_err(|e| {
        error!("Validation error: {}", e);
        HttpError::BadRequest(e.to_string())
    })?;

    let task_id =
        Uuid::from_str(path.id.as_str()).map_err(|e| HttpError::BadRequest(e.to_string()))?;

    let sub_task_id = Uuid::from_str(path.subtask_id.as_str())
        .map_err(|e| HttpError::BadRequest(e.to_string()))?;

    let command = UpdateSubTaskCommand {
        user_id: user.user_id,
        task_id,
        sub_task_id,
        completed: payload.completed,
    };

    debug!("Executing complete subtask command: {:?}", command);

    state.update_subtask_uc.execute(command).await?;

    Ok(Json(UpdateSubTaskResponseDto {
        id: sub_task_id.to_string(),
    }))
}
