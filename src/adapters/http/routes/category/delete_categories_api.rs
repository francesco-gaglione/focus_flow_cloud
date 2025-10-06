use crate::adapters::http::app_state::AppState;
use crate::adapters::http::dto::category_api::delete_categories::{
    DeleteCategoriesDto, DeleteCategoriesResponseDto,
};
use crate::adapters::openapi::CATEGORY_TAG;
use crate::application::app_error::{AppError, AppResult};
use axum::Json;
use axum::extract::State;
use tracing::{debug, error};
use uuid::Uuid;
use validator::Validate;

#[utoipa::path(
    delete,
    path = "/category/deleteCategories",
    tag = CATEGORY_TAG,
    request_body = DeleteCategoriesDto,
    description = "Delete categories and their tasks",
    responses(
        (status = 200, description = "Categories deleted successfully", body = DeleteCategoriesResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_categories_api(
    State(state): State<AppState>,
    Json(payload): Json<DeleteCategoriesDto>,
) -> AppResult<Json<DeleteCategoriesResponseDto>> {
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let res = state
        .category_use_cases
        .delete_categories(
            payload
                .category_ids
                .iter()
                .map(|id| Uuid::parse_str(id).unwrap()) // should be safe due to validation
                .collect(),
        )
        .await?;

    debug!("{:?}", res);

    Ok(Json(DeleteCategoriesResponseDto {
        deleted_ids: res.iter().map(|id| id.to_string()).collect(),
    }))
}
