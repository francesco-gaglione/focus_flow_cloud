use crate::http::app_state::AppState;
use crate::http::dto::validators::validate_uuid::validate_uuid;
use crate::http_error::{HttpError, HttpResult};
use crate::openapi::CATEGORY_TAG;
use axum::extract::{Path, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::{debug, error};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct DeleteCategoriesDto {
    #[validate(custom(function = "validate_uuid"))]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteCategoriesResponseDto {
    pub deleted_ids: Vec<String>,
}

#[utoipa::path(
    delete,
    path = "/api/category/{id}",
    tag = CATEGORY_TAG,
    summary = "Delete a category by id",
    description = "Delete a category and all its tasks by id",
    params(
        ("id" = String, Path, description = "Category ID to delete")
    ),
    responses(
        (status = 200, description = "Category deleted successfully", body = DeleteCategoriesResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
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
        .delete_categories_usecase
        .execute(category_ids)
        .await?;

    debug!("Deleted {} categories", res.len());

    Ok(Json(DeleteCategoriesResponseDto {
        deleted_ids: res.iter().map(|id| id.to_string()).collect(),
    }))
}
