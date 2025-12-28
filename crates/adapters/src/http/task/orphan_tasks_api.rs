use crate::http::app_state::AppState;
use crate::http::dto::task_api::orphan_tasks::OrphanTasksResponseDto;
use crate::http_error::HttpResult;
use crate::mappers::task_mapper::TaskMapper;
use crate::openapi::TASK_TAG;
use axum::extract::State;
use axum::Json;

#[utoipa::path(
    get,
    path = "/api/tasks/orphans",
    tag = TASK_TAG,
    summary = "Get all orphan tasks (tasks without a category)",
    responses(
        (status = 200, description = "Orphan tasks fetched successfully", body = OrphanTasksResponseDto),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn fetch_orphan_tasks_api(
    State(state): State<AppState>,
) -> HttpResult<Json<OrphanTasksResponseDto>> {
    let res = state.orphan_tasks_usecase.execute().await?;
    Ok(Json(OrphanTasksResponseDto {
        orphan_tasks: res
            .iter()
            .map(|task| TaskMapper::entity_to_dto(task.clone()))
            .collect(),
    }))
}
