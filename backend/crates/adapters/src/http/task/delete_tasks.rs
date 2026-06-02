use crate::http::app_state::AppState;
use crate::http::validators::validate_uuid::validate_uuid;
use crate::http_error::{HttpError, HttpResult};
use crate::openapi::TASK_TAG;
use application::use_cases::task::delete_task::DeleteTaskError;
use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

impl From<DeleteTaskError> for HttpError {
    fn from(value: DeleteTaskError) -> Self {
        match value {
            DeleteTaskError::PersistenceError(e) => HttpError::GenericError(e.to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct DeleteTasksDto {
    #[validate(custom(function = "validate_uuid"))]
    pub task_id: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct DeleteTaskResponseDto {
    pub deleted_id: String,
}

#[utoipa::path(
    delete,
    path = "/api/task",
    tag = TASK_TAG,
    summary = "Delete one or more tasks",
    request_body = DeleteTasksDto,
    responses(
        (status = 200, description = "Tasks deleted successfully", body = DeleteTaskResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn delete_tasks_api(
    State(state): State<AppState>,
    Query(payload): Query<DeleteTasksDto>,
) -> HttpResult<Json<DeleteTaskResponseDto>> {
    payload
        .validate()
        .map_err(|e| HttpError::BadRequest(e.to_string()))?;

    let res = state
        .delete_tasks_uc
        .execute(
            payload.task_id.parse().unwrap(), // should be safe due to dto validation
        )
        .await?;

    Ok(Json(DeleteTaskResponseDto {
        deleted_id: res.to_string(),
    }))
}
