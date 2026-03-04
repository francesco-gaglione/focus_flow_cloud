use crate::http::dto::common::task_dto::TaskDto;
use crate::http_error::{map_persistence_error, HttpResult};
use crate::openapi::TASK_TAG;
use crate::{http::app_state::AppState, http_error::HttpError};
use application::use_cases::task::orphan_tasks::{GetOrphanTasksCommand, OrphanTasksError};
use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

impl From<OrphanTasksError> for HttpError {
    fn from(err: OrphanTasksError) -> Self {
        match err {
            OrphanTasksError::PersistenceError(e) => map_persistence_error(e),
        }
    }
}

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct GetOrphanTasksParams {
    pub completed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrphanTasksResponseDto {
    pub orphan_tasks: Vec<TaskDto>,
}

#[utoipa::path(
    get,
    path = "/api/task/orphans",
    tag = TASK_TAG,
    summary = "Get all orphan tasks (tasks without a category)",
    params(
        GetOrphanTasksParams
    ),
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
    Query(params): Query<GetOrphanTasksParams>,
) -> HttpResult<Json<OrphanTasksResponseDto>> {
    let res = state
        .orphan_tasks_usecase
        .execute(GetOrphanTasksCommand {
            completed: params.completed,
        })
        .await?;
    Ok(Json(OrphanTasksResponseDto {
        orphan_tasks: res.iter().map(|task| task.into()).collect(),
    }))
}
