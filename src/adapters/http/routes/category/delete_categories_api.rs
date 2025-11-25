use crate::adapters::http::app_state::AppState;
use crate::adapters::http::dto::category_api::delete_categories::{
    DeleteCategoriesDto, DeleteCategoriesResponseDto,
};
use crate::adapters::http_error::{HttpError, HttpResult};
use crate::adapters::openapi::CATEGORY_TAG;
use axum::extract::{Path, State};
use axum::Json;
use tracing::{debug, error};
use uuid::Uuid;
use validator::Validate;

#[utoipa::path(
    delete,
    path = "/api/categories/{id}",
    tag = CATEGORY_TAG,
    summary = "Delete a category by id",
    description = "Delete a category and all its tasks by id",
    params(
        ("id" = String, Path, description = "Category ID to delete")
    ),
    responses(
        (status = 200, description = "Category deleted successfully", body = DeleteCategoriesResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_categories_api(
    State(state): State<AppState>,
    Path(payload): Path<DeleteCategoriesDto>,
) -> HttpResult<Json<DeleteCategoriesResponseDto>> {
    payload
        .validate()
        .map_err(|e| HttpError::BadRequest(e.to_string()))?;

    let category_id = Uuid::parse_str(&payload.id).map_err(|e| {
        error!("Invalid category ID: {}", e);
        HttpError::BadRequest("Invalid id".to_string())
    })?;

    let category_ids = vec![category_id];

    let res = state
        .category_use_cases
        .delete_categories(category_ids)
        .await?;

    debug!("Deleted {} categories", res.len());

    Ok(Json(DeleteCategoriesResponseDto {
        deleted_ids: res.iter().map(|id| id.to_string()).collect(),
    }))
}
