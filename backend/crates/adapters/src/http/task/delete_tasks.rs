use crate::http::app_state::AppState;
use crate::http::dto::validators::validate_uuids::validate_uuids;
use crate::http_error::{HttpError, HttpResult};
use crate::openapi::TASK_TAG;
use application::use_cases::task::delete_tasks::DeleteTasksError;
use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

impl From<DeleteTasksError> for HttpError {
    fn from(value: DeleteTasksError) -> Self {
        match value {
            DeleteTasksError::PersistenceError(e) => HttpError::GenericError(e.to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeleteTasksDto {
    #[validate(custom(function = "validate_uuids"))]
    pub task_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteTasksResponseDto {
    pub deleted_ids: Vec<String>,
}

#[utoipa::path(
    delete,
    path = "/api/task",
    tag = TASK_TAG,
    summary = "Delete one or more tasks",
    request_body = DeleteTasksDto,
    responses(
        (status = 200, description = "Tasks deleted successfully", body = DeleteTasksResponseDto),
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
    Json(payload): Json<DeleteTasksDto>,
) -> HttpResult<Json<DeleteTasksResponseDto>> {
    payload
        .validate()
        .map_err(|e| HttpError::BadRequest(e.to_string()))?;

    let res = state
        .delete_tasks_usecase
        .execute(
            payload
                .task_ids
                .iter()
                .map(|id| id.parse().unwrap()) // should be safe due to dto validation
                .collect(),
        )
        .await?;

    match !res.is_empty() {
        true => Ok(Json(DeleteTasksResponseDto {
            deleted_ids: res.iter().map(|id| id.to_string()).collect(),
        })),
        false => Err(HttpError::GenericError("Tasks not delted".to_string())),
    }
}
