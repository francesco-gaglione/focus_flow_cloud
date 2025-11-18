use crate::adapters::http::app_state::AppState;
use crate::adapters::http::dto::category_api::get_category::GetCategoryResponseDto;
use crate::adapters::http::dto::common::category_dto::CategoryDto;
use crate::adapters::openapi::CATEGORY_TAG;
use crate::application::app_error::{AppError, AppResult};
use axum::extract::{Path, State};
use axum::Json;

#[utoipa::path(
    get,
    path = "/api/categories/{id}",
    tag = CATEGORY_TAG,
    responses(
        (status = 200, description = "Category", body = GetCategoryResponseDto),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_category(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<GetCategoryResponseDto>> {
    let id = id
        .parse()
        .map_err(|_| AppError::Database("Invalid id".to_string()))?;
    let category = state.category_use_cases.get_category(id).await?;

    let response = GetCategoryResponseDto {
        category: CategoryDto {
            id: category.id.to_string(),
            name: category.name,
            description: category.description,
            color: category.color,
            tasks: vec![], //TODO
        },
    };

    tracing::info!("Category retrieved successfully: {:?}", response);

    Ok(Json(response))
}
