use crate::adapters::http::app_state::AppState;
use crate::adapters::http::dto::category_api::update_category::{
    UpdateCategoryDto, UpdateCategoryPathDto, UpdateCategoryResponseDto,
};
use crate::adapters::http::dto::common::category_dto::CategoryDto;
use crate::adapters::http_error::{HttpError, HttpResult};
use crate::adapters::openapi::CATEGORY_TAG;
use application::use_cases::category::command::update_category::UpdateCategoryCommand;
use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;
use validator::Validate;

#[utoipa::path(
    put,
    path = "/api/categories/{id}",
    tag = CATEGORY_TAG,
    summary = "Update a category",
    params(
        ("id" = String, Path, description = "Category ID to update")
    ),
    request_body = UpdateCategoryDto,
    responses(
        (status = 200, description = "Category updated successfully", body = UpdateCategoryResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 404, description = "Category not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_category_api(
    State(state): State<AppState>,
    Path(path): Path<UpdateCategoryPathDto>,
    Json(payload): Json<UpdateCategoryDto>,
) -> HttpResult<Json<UpdateCategoryResponseDto>> {
    path.validate()
        .map_err(|e| HttpError::BadRequest(e.to_string()))?;

    payload
        .validate()
        .map_err(|e| HttpError::BadRequest(e.to_string()))?;

    let category_id = Uuid::parse_str(&path.id)
        .map_err(|_| HttpError::BadRequest("Category id malformed".to_string()))?;

    let category = state
        .update_category_usecase
        .execute(UpdateCategoryCommand {
            id: category_id,
            name: payload.name,
            description: payload.description,
            color: payload.color,
        })
        .await?;

    Ok(Json(UpdateCategoryResponseDto {
        updated_category: CategoryDto {
            id: category.id().to_string(),
            name: category.name().to_string(),
            description: category.description().map(|s| s.to_string()),
            color: category.color().to_string(),
            tasks: Vec::new(), //TODO should return tasks?
        },
    }))
}
