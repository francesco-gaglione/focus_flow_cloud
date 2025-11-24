use crate::adapters::http::app_state::AppState;
use crate::adapters::http::dto::task_api::create_task::{CreateTaskDto, CreateTaskResponseDto};
use crate::adapters::mappers::task_mapper::TaskMapper;
use crate::adapters::openapi::TASK_TAG;
use crate::application::app_error::{AppError, AppResult};
use axum::extract::State;
use axum::Json;
use validator::Validate;

#[utoipa::path(
    post,
    path = "/api/tasks",
    tag = TASK_TAG,
    summary = "Create a new task",
    request_body = CreateTaskDto,
    responses(
        (status = 201, description = "Task created successfully", body = CreateTaskResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 409, description = "Task already exists"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_task_api(
    State(state): State<AppState>,
    Json(payload): Json<CreateTaskDto>,
) -> AppResult<Json<CreateTaskResponseDto>> {
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let command = TaskMapper::create_dto_to_command(payload)?;

    let id = state.task_use_cases.create_task(command).await?;

    Ok(Json(CreateTaskResponseDto { id: id.to_string() }))
}
