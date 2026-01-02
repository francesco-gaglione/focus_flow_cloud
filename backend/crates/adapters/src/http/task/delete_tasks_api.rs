use crate::http::app_state::AppState;
use crate::http::dto::task_api::delete_task::{DeleteTasksDto, DeleteTasksResponseDto};
use crate::http_error::{HttpError, HttpResult};
use crate::openapi::TASK_TAG;
use axum::extract::State;
use axum::Json;
use validator::Validate;

#[utoipa::path(
    delete,
    path = "/api/tasks",
    tag = TASK_TAG,
    summary = "Delete one or more tasks",
    request_body = DeleteTasksDto,
    responses(
        (status = 200, description = "Tasks deleted successfully", body = DeleteTasksResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 500, description = "Internal server error")
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
