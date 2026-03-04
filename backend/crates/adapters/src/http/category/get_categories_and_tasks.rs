use crate::http::dto::common::category_dto::CategoryDto;
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
}

#[utoipa::path(
    get,
    path = "/api/category",
    tag = CATEGORY_TAG,
    summary = "Get all categories and their tasks",
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
    let categories_and_tasks = state
        .get_category_and_task_usecase
        .execute(GetCategoryAndTasksCommand {
            include_completed_tasks: params.include_completed_tasks,
        })
        .await?;

    let response = GetCategoriesResponseDto {
        categories: categories_and_tasks
            .category_with_tasks
            .into_iter()
            .map(|c| CategoryDto {
                id: c.category.id.to_string(),
                name: c.category.name.to_string(),
                description: c.category.description.map(|s| s.to_string()),
                color: c.category.color.to_string(),
                tasks: c.tasks.iter().map(|t| t.into()).collect(),
            })
            .collect(),
    };

    Ok(Json(response))
}
