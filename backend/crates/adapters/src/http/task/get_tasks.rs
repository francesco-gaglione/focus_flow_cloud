use crate::http::dto::common::task_dto::TaskDto;
use crate::http_error::{map_persistence_error, HttpResult};
use crate::openapi::TASK_TAG;
use crate::{http::app_state::AppState, http_error::HttpError};
use application::use_cases::task::get_tasks::{GetTaskError, GetTasksCommand};
use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

impl From<GetTaskError> for HttpError {
    fn from(err: GetTaskError) -> Self {
        match err {
            GetTaskError::PersistenceError(e) => map_persistence_error(e),
        }
    }
}

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct GetTasksParams {
    pub completed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TasksResponseDto {
    pub tasks: Vec<TaskDto>,
}

#[utoipa::path(
    get,
    path = "/api/task",
    tag = TASK_TAG,
    summary = "Get all tasks",
    params(
        GetTasksParams
    ),
    responses(
        (status = 200, description = "Tasks fetched successfully", body = TasksResponseDto),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn get_tasks_api(
    State(state): State<AppState>,
    Query(params): Query<GetTasksParams>,
) -> HttpResult<Json<TasksResponseDto>> {
    let res = state
        .get_tasks_usecase
        .execute(GetTasksCommand {
            completed: params.completed,
        })
        .await?;
    Ok(Json(TasksResponseDto {
        tasks: res.iter().map(|task| task.into()).collect(),
    }))
}
