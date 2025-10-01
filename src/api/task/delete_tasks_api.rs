use crate::api::api_error::ApiError;
use crate::config::app_state::AppState;
use crate::config::openapi::TASK_TAG;
use crate::dto::task_api::delete_task::{DeleteTasksDto, DeleteTasksResponseDto};
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
) -> Result<Json<DeleteTasksResponseDto>, ApiError> {
    payload.validate()?;

    let res = state
        .task_service
        .delete_tasks(
            payload
                .task_ids
                .iter()
                .map(|id| id.parse().unwrap()) // should be safe due to dto validation
                .collect(),
        )
        .await;

    log::debug!("{:?}", res);

    match res {
        Ok(res) => Ok(Json(DeleteTasksResponseDto {
            deleted_ids: res.iter().map(|id| id.to_string()).collect(),
        })),
        Err(err) => {
            log::error!("{:?}", err);
            Err(ApiError::from(err))
        }
    }
}
