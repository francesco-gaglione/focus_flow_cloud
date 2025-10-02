use crate::AppState;
use crate::api::api_error::ApiError;
use crate::dto::category_api::get_categories::GetCategoriesResponseDto;
use axum::Json;
use axum::extract::State;
use crate::config::openapi::CATEGORY_TAG;

#[utoipa::path(
    get,
    path = "/category/getCategoriesAndTasks",
    tag = CATEGORY_TAG,
    responses(
        (status = 200, description = "Category list", body = GetCategoriesResponseDto),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_categories_and_tasks_api(
    State(state): State<AppState>,
) -> Result<Json<GetCategoriesResponseDto>, ApiError> {
    let categories_and_tasks = state
        .category_service
        .get_all_categories_and_tasks()
        .await?;

    let response = GetCategoriesResponseDto {
        categories: categories_and_tasks
            .categories
            .into_iter()
            .map(|c| c.into())
            .collect(),
        orphan_tasks: categories_and_tasks
            .orphan_tasks
            .into_iter()
            .map(|t| (&t).into())
            .collect(),
    };

    Ok(Json(response))
}
