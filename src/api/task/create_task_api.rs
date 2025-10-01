use crate::AppState;
use crate::api::api_error::ApiError;
use crate::config::openapi::TASK_TAG;
use crate::dto::task_api::create_task::{CreateTaskDto, CreateTaskResponseDto};
use crate::services::task_service::CreateTaskCommand;
use axum::Json;
use axum::extract::State;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use validator::Validate;

#[utoipa::path(
    post,
    path = "/task/createTask",
    tag = TASK_TAG,
    request_body = CreateTaskDto,
    responses(
        (status = 200, description = "Task created successfully", body = CreateTaskResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 409, description = "Task already exists"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_task_api(
    State(state): State<AppState>,
    Json(payload): Json<CreateTaskDto>,
) -> Result<Json<CreateTaskResponseDto>, ApiError> {
    log::debug!("{:?}", payload);
    payload.validate()?;

    let scheduled_date = match payload.scheduled_date {
        Some(timestamp) => {
            let date_time = DateTime::from_timestamp(timestamp, 0)
                .ok_or_else(|| ApiError::BadRequest("Invalid timestamp".to_string()))?;
            Some(date_time.date_naive())
        }
        None => None,
    };

    let task = state
        .task_service
        .create_task(CreateTaskCommand {
            name: payload.name,
            description: payload.description,
            category_id: payload.category_id,
            scheduled_date,
        })
        .await?;

    Ok(Json(CreateTaskResponseDto {
        id: task.id.to_string(),
    }))
}
