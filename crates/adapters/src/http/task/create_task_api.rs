use crate::http::app_state::AppState;
use crate::http::dto::task_api::create_task::{CreateTaskDto, CreateTaskResponseDto};
use crate::http_error::{HttpError, HttpResult};
use crate::mappers::task_mapper::TaskMapper;
use crate::openapi::TASK_TAG;
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
) -> HttpResult<Json<CreateTaskResponseDto>> {
    payload
        .validate()
        .map_err(|e| HttpError::BadRequest(e.to_string()))?;

    let command = TaskMapper::create_dto_to_command(payload)?;

    let id = state.create_task_usecase.execute(command).await?;

    Ok(Json(CreateTaskResponseDto { id: id.to_string() }))
}
