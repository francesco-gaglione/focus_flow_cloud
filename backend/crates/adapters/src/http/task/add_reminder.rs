use crate::http::app_state::AppState;
use crate::http::model::session_model::UserSession;
use crate::http_error::{HttpError, HttpResult};
use crate::openapi::TASK_TAG;
use application::use_cases::task::add_reminder::{AddReminderToTaskCommand, AddReminderToTaskError};
use axum::extract::{Path, State};
use axum::{Extension, Json};
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use tracing::error;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

impl From<AddReminderToTaskError> for HttpError {
    fn from(value: AddReminderToTaskError) -> Self {
        match value {
            AddReminderToTaskError::PersistenceError(e) => HttpError::GenericError(e.to_string()),
            AddReminderToTaskError::TaskNotFound(uuid) => {
                HttpError::NotFound(format!("Task {} not found", uuid))
            }
            AddReminderToTaskError::Unauthorized => HttpError::Forbidden,
            AddReminderToTaskError::WorkerPortError(e) => HttpError::GenericError(e.to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct AddReminderDto {
    pub date: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
pub struct AddReminderResponseDto {
    pub id: String,
}

#[utoipa::path(
    post,
    path = "/api/task/{id}/reminder",
    tag = TASK_TAG,
    summary = "Add a reminder to a task",
    request_body = AddReminderDto,
    params(
        ("id" = String, Path, description = "Task ID")
    ),
    responses(
        (status = 201, description = "Reminder added successfully", body = AddReminderResponseDto),
        (status = 400, description = "Bad request - invalid date"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Task not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn add_reminder_api(
    State(state): State<AppState>,
    Extension(user): Extension<UserSession>,
    Path(task_id): Path<Uuid>,
    Json(payload): Json<AddReminderDto>,
) -> HttpResult<Json<AddReminderResponseDto>> {
    let date = DateTime::from_timestamp(payload.date, 0)
        .ok_or_else(|| HttpError::BadRequest("Invalid date timestamp".to_string()))?;

    let command = AddReminderToTaskCommand {
        task_id,
        user_id: user.user_id,
        date,
    };

    let reminder_id = state
        .add_reminder_uc
        .execute(command)
        .await
        .map_err(|e| {
            error!("Error adding reminder: {}", e);
            HttpError::from(e)
        })?;

    Ok(Json(AddReminderResponseDto {
        id: reminder_id.to_string(),
    }))
}
