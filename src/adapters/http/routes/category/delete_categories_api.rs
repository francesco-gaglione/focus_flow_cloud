use std::path::Path;

use crate::adapters::http::app_state::AppState;
use crate::adapters::http::dto::category_api::delete_categories::{
    DeleteCategoriesDto, DeleteCategoriesResponseDto,
};
use crate::adapters::openapi::CATEGORY_TAG;
use crate::application::app_error::{AppError, AppResult};
use axum::extract::{Query, State};
use axum::Json;
use tracing::debug;
use uuid::Uuid;
use validator::Validate;

#[utoipa::path(
    delete,
    path = "/categories",
    tag = CATEGORY_TAG,
    description = "Delete multiple categories and their tasks",
    params(
        ("id" = Vec<String>, Query, description = "Category IDs to delete (repeat for multiple)")
    ),
    responses(
        (status = 200, description = "Categories deleted successfully", body = DeleteCategoriesResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_categories_api(
    State(state): State<AppState>,
    Query(payload): Query<DeleteCategoriesDto>,
) -> AppResult<Json<DeleteCategoriesResponseDto>> {
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let category_ids: Vec<Uuid> = payload
        .category_ids
        .iter()
        .map(|id| Uuid::parse_str(id))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| AppError::BadRequest(format!("Invalid UUID format: {}", e)))?;

    let res = state
        .category_use_cases
        .delete_categories(category_ids)
        .await?;

    debug!("Deleted {} categories", res.len());

    Ok(Json(DeleteCategoriesResponseDto {
        deleted_ids: res.iter().map(|id| id.to_string()).collect(),
    }))
}
