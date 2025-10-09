use crate::adapters::http::app_state::AppState;
use crate::adapters::http::dto::task_api::update_task::{UpdateTaskDto, UpdateTaskResponseDto};
use crate::adapters::mappers::task_mapper::TaskMapper;
use crate::adapters::openapi::TASK_TAG;
use crate::application::app_error::{AppError, AppResult};
use axum::Json;
use axum::extract::State;
use validator::Validate;

#[utoipa::path(
    put,
    path = "/task/updateTask",
    tag = TASK_TAG,
    request_body = UpdateTaskDto,
    responses(
        (status = 200, description = "Task updated successfully", body = UpdateTaskResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_task_api(
    State(state): State<AppState>,
    Json(payload): Json<UpdateTaskDto>,
) -> AppResult<Json<UpdateTaskResponseDto>> {
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let command = TaskMapper::update_dto_to_command(payload)?;

    let task = state.task_use_cases.update_task(command).await?;

    let task_dto = TaskMapper::entity_to_dto(task);

    Ok(Json(UpdateTaskResponseDto {
        updated_task: task_dto,
    }))
}
