use crate::http::app_state::AppState;
use crate::http::dto::task_api::update_task::{
    UpdateTaskDto, UpdateTaskPathDto, UpdateTaskResponseDto,
};
use crate::http_error::{HttpError, HttpResult};
use crate::mappers::task_mapper::TaskMapper;
use crate::openapi::TASK_TAG;
use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;
use validator::Validate;

#[utoipa::path(
    put,
    path = "/api/tasks/{id}",
    tag = TASK_TAG,
    summary = "Update a task",
    params(
        ("id" = String, Path, description = "Task ID to update")
    ),
    request_body = UpdateTaskDto,
    responses(
        (status = 200, description = "Task updated successfully", body = UpdateTaskResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 404, description = "Task not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_task_api(
    State(state): State<AppState>,
    Path(path): Path<UpdateTaskPathDto>,
    Json(payload): Json<UpdateTaskDto>,
) -> HttpResult<Json<UpdateTaskResponseDto>> {
    path.validate()
        .map_err(|e| HttpError::BadRequest(e.to_string()))?;

    payload
        .validate()
        .map_err(|e| HttpError::BadRequest(e.to_string()))?;

    let task_id = Uuid::parse_str(&path.id)
        .map_err(|_| HttpError::BadRequest("Task id malformed".to_string()))?;

    let command = TaskMapper::update_dto_to_command(task_id, payload)?;

    let task = state.update_task_usecase.execute(command).await?;

    let task_dto = TaskMapper::entity_to_dto(task);

    Ok(Json(UpdateTaskResponseDto {
        updated_task: task_dto,
    }))
}
