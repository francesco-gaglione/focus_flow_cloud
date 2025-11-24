use crate::adapters::http::app_state::AppState;
use crate::adapters::http::dto::category_api::get_categories::GetCategoriesResponseDto;
use crate::adapters::http::dto::common::category_dto::CategoryDto;
use crate::adapters::mappers::task_mapper::TaskMapper;
use crate::adapters::openapi::CATEGORY_TAG;
use crate::application::app_error::AppResult;
use axum::extract::State;
use axum::Json;

#[utoipa::path(
    get,
    path = "/api/categories",
    tag = CATEGORY_TAG,
    responses(
        (status = 200, description = "Category list", body = GetCategoriesResponseDto),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_categories_and_tasks_api(
    State(state): State<AppState>,
) -> AppResult<Json<GetCategoriesResponseDto>> {
    let categories_and_tasks = state
        .category_use_cases
        .get_all_category_and_tasks()
        .await?;

    let response = GetCategoriesResponseDto {
        categories: categories_and_tasks
            .category_with_tasks
            .into_iter()
            .map(|c| CategoryDto {
                id: c.category.id.to_string(),
                name: c.category.name,
                description: c.category.description,
                color: c.category.color,
                tasks: TaskMapper::entities_to_dtos(c.tasks),
            })
            .collect(),
    };

    Ok(Json(response))
}
