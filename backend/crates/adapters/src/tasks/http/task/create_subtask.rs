use crate::shared::http::app_state::AppState;
use crate::shared::http::model::session_model::UserSession;
use crate::http_error::{HttpError, HttpResult};
use crate::openapi::TASK_TAG;
use application::tasks::use_cases::task::add_subtask::{AddSubTaskCommand, AddSubTaskError};
use axum::extract::{Path, State};
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use tracing::error;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct CreateSubtaskDto {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Title must be between 1 and 255 characters"
    ))]
    pub title: String,
    #[validate(length(max = 255, message = "Description must not exceed 255 characters"))]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
pub struct CreateSubtaskResponseDto {
    pub id: String,
}

impl From<AddSubTaskError> for HttpError {
    fn from(value: AddSubTaskError) -> Self {
        match value {
            AddSubTaskError::PersistenceError(e) => HttpError::GenericError(e.to_string()),
            AddSubTaskError::TaskNotFound(uuid) => {
                HttpError::NotFound(format!("Task {} not found", uuid))
            }
            AddSubTaskError::Unauthorized => HttpError::Forbidden,
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/task/{id}/subtask",
    tag = TASK_TAG,
    summary = "Add a subtask to a task",
    request_body = CreateSubtaskDto,
    responses(
        (status = 201, description = "Subtask created successfully", body = CreateSubtaskResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Task not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn create_subtask_api(
    State(state): State<AppState>,
    Extension(user): Extension<UserSession>,
    Path(task_id): Path<Uuid>,
    Json(payload): Json<CreateSubtaskDto>,
) -> HttpResult<Json<CreateSubtaskResponseDto>> {
    payload.validate().map_err(|e| {
        error!("Validation error: {}", e);
        HttpError::BadRequest(e.to_string())
    })?;

    let command = AddSubTaskCommand {
        task_id,
        user_id: user.user_id,
        title: payload.title,
        description: payload.description,
    };

    let subtask_id = state.add_subtask_uc.execute(command).await?;

    Ok(Json(CreateSubtaskResponseDto {
        id: subtask_id.to_string(),
    }))
}
