use crate::http::dto::common::task_dto::{self, TaskDto};
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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct TasksResponseDto {
    pub completed: Vec<TaskDto>,
    pub today: Vec<TaskDto>,
    pub next_week: Vec<TaskDto>,
    pub unscheduled: Vec<TaskDto>,
    pub incoming: Vec<TaskDto>,
}

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct GetTasksParams {
    pub completed: bool,
    pub today: bool,
    pub next_week: bool,
    pub unscheduled: bool,
    pub incoming: bool,
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
    tracing::info!("Fetching tasks with completed: {:?}", params.completed);
    let res = state
        .get_tasks_uc
        .execute(GetTasksCommand {
            completed: params.completed,
            today: params.today,
            next_week: params.next_week,
            unscheduled: params.unscheduled,
            incoming: params.incoming,
        })
        .await?;
    tracing::info!("Fetched tasks");
    Ok(Json(TasksResponseDto {
        completed: res
            .completed
            .iter()
            .map(task_dto::from_task_output)
            .collect(),
        today: res.today.iter().map(task_dto::from_task_output).collect(),
        next_week: res
            .next_week
            .iter()
            .map(task_dto::from_task_output)
            .collect(),
        unscheduled: res
            .unscheduled
            .iter()
            .map(task_dto::from_task_output)
            .collect(),
        incoming: res
            .incoming
            .iter()
            .map(task_dto::from_task_output)
            .collect(),
    }))
}
