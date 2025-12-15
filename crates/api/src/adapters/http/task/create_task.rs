use crate::adapters::http::app_state::AppState;
use crate::adapters::http::dto::validators::validate_uuid::validate_uuid;
use crate::adapters::http_error::{HttpError, HttpResult};
use crate::adapters::mappers::task_mapper::TaskMapper;
use crate::adapters::openapi::TASK_TAG;
use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskDto {
    #[validate(custom(function = "validate_uuid"))]
    pub category_id: Option<String>,

    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: String,

    #[validate(length(max = 255, message = "Description must not exceed 255 characters"))]
    pub description: Option<String>,

    pub scheduled_date: Option<i64>, //timestamp in seconds
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateTaskResponseDto {
    pub id: String,
}

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
