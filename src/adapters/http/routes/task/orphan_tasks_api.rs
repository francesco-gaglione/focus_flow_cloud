use crate::adapters::http::app_state::AppState;
use crate::adapters::http::dto::task_api::orphan_tasks::OrphanTasksResponseDto;
use crate::adapters::mappers::task_mapper::TaskMapper;
use crate::adapters::openapi::TASK_TAG;
use crate::application::app_error::{AppError, AppResult};
use axum::extract::State;
use axum::Json;

#[utoipa::path(
    get,
    path = "/task/orphanTasks",
    tag = TASK_TAG,
    responses(
        (status = 200, description = "Orphan tasks fetched successfully", body = OrphanTasksResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn fetch_orphan_tasks_api(
    State(state): State<AppState>,
) -> AppResult<Json<OrphanTasksResponseDto>> {
    let res = state.task_use_cases.orphan_tasks().await?;

    match res.len() > 0 {
        true => Ok(Json(OrphanTasksResponseDto {
            orphan_tasks: res
                .iter()
                .map(|task| TaskMapper::entity_to_dto(task.clone()))
                .collect(),
        })),
        false => Err(AppError::GenericError("Tasks not found".to_string())),
    }
}
