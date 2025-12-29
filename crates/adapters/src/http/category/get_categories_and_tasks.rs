use crate::http::app_state::AppState;
use crate::http::dto::common::category_dto::CategoryDto;
use crate::http_error::HttpResult;
use crate::mappers::task_mapper::TaskMapper;
use crate::openapi::CATEGORY_TAG;
use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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
) -> HttpResult<Json<GetCategoriesResponseDto>> {
    let categories_and_tasks = state.get_category_and_task_usecase.execute().await?;

    let response = GetCategoriesResponseDto {
        categories: categories_and_tasks
            .category_with_tasks
            .into_iter()
            .map(|c| CategoryDto {
                id: c.category.id().to_string(),
                name: c.category.name().to_string(),
                description: c.category.description().map(|s| s.to_string()),
                color: c.category.color().to_string(),
                tasks: TaskMapper::entities_to_dtos(c.tasks),
            })
            .collect(),
    };

    Ok(Json(response))
}
