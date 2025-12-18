use crate::adapters::http::app_state::AppState;
use crate::adapters::http::dto::common::task_dto::TaskDto;
use crate::adapters::http_error::HttpResult;
use crate::adapters::mappers::task_mapper::TaskMapper;
use crate::adapters::openapi::TASK_TAG;
use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrphanTasksResponseDto {
    pub orphan_tasks: Vec<TaskDto>,
}

#[utoipa::path(
    get,
    path = "/api/tasks/orphans",
    tag = TASK_TAG,
    summary = "Get all orphan tasks (tasks without a category)",
    responses(
        (status = 200, description = "Orphan tasks fetched successfully", body = OrphanTasksResponseDto),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
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
