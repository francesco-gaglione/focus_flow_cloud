use crate::http::app_state::AppState;
use crate::http::model::session_model::UserSession;
use crate::http_error::{HttpError, HttpResult};
use crate::openapi::TASK_TAG;
use application::use_cases::task::delete_reminder::{DeleteReminderCommand, DeleteReminderError};
use axum::extract::{Path, State};
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use tracing::error;
use utoipa::ToSchema;
use uuid::Uuid;

impl From<DeleteReminderError> for HttpError {
    fn from(value: DeleteReminderError) -> Self {
        match value {
            DeleteReminderError::PersistenceError(e) => HttpError::GenericError(e.to_string()),
            DeleteReminderError::TaskNotFound(uuid) => {
                HttpError::NotFound(format!("Task {} not found", uuid))
            }
            DeleteReminderError::ReminderNotFound(uuid) => {
                HttpError::NotFound(format!("Reminder {} not found", uuid))
            }
            DeleteReminderError::Unauthorized => HttpError::Forbidden,
            DeleteReminderError::WorkerPortError(e) => HttpError::GenericError(e.to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteReminderResponseDto {
    pub success: bool,
}

#[utoipa::path(
    delete,
    path = "/api/task/{id}/reminder/{reminder_id}",
    tag = TASK_TAG,
    summary = "Delete a reminder from a task",
    params(
        ("id" = String, Path, description = "Task ID"),
        ("reminder_id" = String, Path, description = "Reminder ID to delete")
    ),
    responses(
        (status = 200, description = "Reminder deleted successfully", body = DeleteReminderResponseDto),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Task or reminder not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn delete_reminder_api(
    State(state): State<AppState>,
    Extension(user): Extension<UserSession>,
    Path((task_id, reminder_id)): Path<(Uuid, Uuid)>,
) -> HttpResult<Json<DeleteReminderResponseDto>> {
    let command = DeleteReminderCommand {
        task_id,
        reminder_id,
        user_id: user.user_id,
    };

    state
        .delete_reminder_uc
        .execute(command)
        .await
        .map_err(|e| {
            error!("Error deleting reminder: {}", e);
            HttpError::from(e)
        })?;

    Ok(Json(DeleteReminderResponseDto { success: true }))
}
