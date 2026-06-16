use crate::http::app_state::AppState;
use crate::http::model::session_model::UserSession;
use crate::http_error::{HttpError, HttpResult};
use crate::openapi::TASK_TAG;
use application::use_cases::task::delete_subtask::{DeleteSubTaskCommand, DeleteSubTaskError};
use axum::extract::{Path, State};
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use tracing::error;
use utoipa::ToSchema;
use uuid::Uuid;

impl From<DeleteSubTaskError> for HttpError {
    fn from(value: DeleteSubTaskError) -> Self {
        match value {
            DeleteSubTaskError::PersistenceError(e) => HttpError::GenericError(e.to_string()),
            DeleteSubTaskError::TaskNotFound(uuid) => {
                HttpError::NotFound(format!("Task {} not found", uuid))
            }
            DeleteSubTaskError::Unauthorized => HttpError::Forbidden,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteSubTaskResponseDto {
    pub success: bool,
}

#[utoipa::path(
    delete,
    path = "/api/task/{id}/subtask/{subtask_id}",
    tag = TASK_TAG,
    summary = "Delete a subtask from a task",
    params(
        ("id" = String, Path, description = "Task ID"),
        ("subtask_id" = String, Path, description = "Subtask ID to delete")
    ),
    responses(
        (status = 200, description = "Subtask deleted successfully", body = DeleteSubTaskResponseDto),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Task not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn delete_subtask_api(
    State(state): State<AppState>,
    Extension(user): Extension<UserSession>,
    Path((task_id, subtask_id)): Path<(Uuid, Uuid)>,
) -> HttpResult<Json<DeleteSubTaskResponseDto>> {
    let command = DeleteSubTaskCommand {
        task_id,
        subtask_id,
        user_id: user.user_id,
    };

    state.delete_subtask_uc.execute(command).await.map_err(|e| {
        error!("Error deleting subtask: {}", e);
        HttpError::from(e)
    })?;

    Ok(Json(DeleteSubTaskResponseDto { success: true }))
}
