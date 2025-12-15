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
pub struct TasksResponseDto {
    pub tasks: Vec<TaskDto>,
}

#[utoipa::path(
    get,
    path = "/api/tasks",
    tag = TASK_TAG,
    summary = "Get all tasks",
    responses(
        (status = 200, description = "Tasks fetched successfully", body = TasksResponseDto),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_tasks_api(State(state): State<AppState>) -> HttpResult<Json<TasksResponseDto>> {
    let res = state.get_tasks_usecase.execute().await?;
    Ok(Json(TasksResponseDto {
        tasks: res
            .iter()
            .map(|task| TaskMapper::entity_to_dto(task.clone()))
            .collect(),
    }))
}
