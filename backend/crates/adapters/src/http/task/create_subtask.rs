use crate::http::app_state::AppState;
use crate::http::model::session_model::UserSession;
use crate::http_error::{HttpError, HttpResult};
use crate::openapi::TASK_TAG;
use application::use_cases::task::add_subtask::{AddSubTaskCommand, AddSubTaskError};
use axum::extract::{Path, State};
use axum::{Extension, Json};
use shared::task::{CreateSubtaskDto, CreateSubtaskResponseDto};
use tracing::error;
use uuid::Uuid;
use validator::Validate;

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
