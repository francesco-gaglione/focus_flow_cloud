use crate::http::dto::common::category_dto::CategoryDto;
use crate::http::dto::common::task_dto::TaskDto;
use crate::http_error::{map_persistence_error, HttpResult};
use crate::openapi::CATEGORY_TAG;
use crate::{http::app_state::AppState, http_error::HttpError};
use application::use_cases::category::get_category_and_task_usecase::{
    GetCategoryAndTasksCommand, GetCategoryAndTasksError,
};
use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

impl From<GetCategoryAndTasksError> for HttpError {
    fn from(err: GetCategoryAndTasksError) -> Self {
        match err {
            GetCategoryAndTasksError::PersistenceError(e) => map_persistence_error(e),
        }
    }
}

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct GetCategoryTasksParams {
    pub include_completed_tasks: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetCategoriesResponseDto {
    pub categories: Vec<CategoryDto>,
    pub tasks: Vec<TaskDto>,
}

#[utoipa::path(
    get,
    path = "/api/category",
    tag = CATEGORY_TAG,
    summary = "Get all categories and tasks",
    params(
        GetCategoryTasksParams
    ),
    responses(
        (status = 200, description = "Category list", body = GetCategoriesResponseDto),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn get_categories_and_tasks_api(
    State(state): State<AppState>,
    Query(params): Query<GetCategoryTasksParams>,
) -> HttpResult<Json<GetCategoriesResponseDto>> {
    let result = state
        .get_category_and_task_uc
        .execute(GetCategoryAndTasksCommand {
            include_completed_tasks: params.include_completed_tasks,
        })
        .await?;

    Ok(Json(GetCategoriesResponseDto {
        categories: result
            .categories
            .iter()
            .map(|c| CategoryDto {
                id: c.id.to_string(),
                name: c.name.clone(),
                description: c.description.clone(),
                color: c.color.clone(),
            })
            .collect(),
        tasks: result.tasks.iter().map(|t| t.into()).collect(),
    }))
}
