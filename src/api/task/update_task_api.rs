use crate::api::api_error::ApiError;
use crate::config::app_state::AppState;
use crate::config::openapi::TASK_TAG;
use crate::dto::task_api::update_task::{UpdateTaskDto, UpdateTaskResponseDto};
use crate::services::task_service::UpdateTaskCommand;
use axum::Json;
use axum::extract::State;
use chrono::DateTime;
use uuid::Uuid;
use validator::Validate;

#[utoipa::path(
    put,
    path = "/task/updateTask",
    tag = TASK_TAG,
    request_body = UpdateTaskDto,
    responses(
        (status = 200, description = "Task deleted successfully", body = UpdateTaskResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_task_api(
    State(state): State<AppState>,
    Json(payload): Json<UpdateTaskDto>,
) -> Result<Json<UpdateTaskResponseDto>, ApiError> {
    payload.validate()?;

    let scheduled_date = match payload.scheduled_date {
        None => None,
        Some(scheduled_date) => {
            let timestamp = DateTime::from_timestamp_secs(scheduled_date);
            timestamp.map(|t| t.date_naive())
        }
    };

    let completed_at = match payload.completed_at {
        None => None,
        Some(completed_at) => {
            let timestamp = DateTime::from_timestamp_secs(completed_at);
            timestamp
        }
    };

    let task = state
        .task_service
        .update_task(UpdateTaskCommand {
            task_id: Uuid::parse_str(payload.task_id.as_str()).unwrap(), // should be safe due to dto validation
            name: payload.name,
            description: payload.description,
            category_id: payload.category_id,
            scheduled_date,
            completed_at,
        })
        .await?;

    Ok(Json(UpdateTaskResponseDto {
        updated_task: (&task).into(),
    }))
}
