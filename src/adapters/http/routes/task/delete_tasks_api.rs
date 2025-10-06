use crate::adapters::http::app_state::AppState;
use crate::adapters::http::dto::task_api::delete_task::{DeleteTasksDto, DeleteTasksResponseDto};
use crate::adapters::openapi::TASK_TAG;
use crate::application::app_error::{AppError, AppResult};
use axum::Json;
use axum::extract::State;
use validator::Validate;

#[utoipa::path(
    delete,
    path = "/task/deleteTasks",
    tag = TASK_TAG,
    request_body = DeleteTasksDto,
    responses(
        (status = 200, description = "Task deleted successfully", body = DeleteTasksResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_tasks_api(
    State(state): State<AppState>,
    Json(payload): Json<DeleteTasksDto>,
) -> AppResult<Json<DeleteTasksResponseDto>> {
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let res = state
        .task_use_cases
        .delete_tasks(
            payload
                .task_ids
                .iter()
                .map(|id| id.parse().unwrap()) // should be safe due to dto validation
                .collect(),
        )
        .await?;

    match res.len() > 0 {
        true => Ok(Json(DeleteTasksResponseDto {
            deleted_ids: res.iter().map(|id| id.to_string()).collect(),
        })),
        false => Err(AppError::GenericError("Tasks not delted".to_string())),
    }
}
