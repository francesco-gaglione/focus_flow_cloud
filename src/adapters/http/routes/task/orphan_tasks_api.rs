use crate::adapters::http::app_state::AppState;
use crate::adapters::http::dto::task_api::orphan_tasks::OrphanTasksResponseDto;
use crate::adapters::mappers::task_mapper::TaskMapper;
use crate::adapters::openapi::TASK_TAG;
use crate::application::app_error::AppResult;
use axum::extract::State;
use axum::Json;

#[utoipa::path(
    get,
    path = "/api/tasks/orphans",
    tag = TASK_TAG,
    responses(
        (status = 200, description = "Orphan tasks fetched successfully", body = OrphanTasksResponseDto),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn fetch_orphan_tasks_api(
    State(state): State<AppState>,
) -> AppResult<Json<OrphanTasksResponseDto>> {
    let res = state.task_use_cases.orphan_tasks().await?;
    Ok(Json(OrphanTasksResponseDto {
        orphan_tasks: res
            .iter()
            .map(|task| TaskMapper::entity_to_dto(task.clone()))
            .collect(),
    }))
}
