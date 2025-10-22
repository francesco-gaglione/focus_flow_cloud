use crate::adapters::http::app_state::AppState;
use crate::adapters::http::dto::category_api::update_category::{
    UpdateCategoryDto, UpdateCategoryResponseDto,
};
use crate::adapters::http::dto::common::category_dto::CategoryDto;
use crate::adapters::openapi::CATEGORY_TAG;
use crate::application::app_error::{AppError, AppResult};
use crate::application::use_cases::commands::update_category::UpdateCategoryCommand;
use axum::extract::State;
use axum::Json;
use uuid::Uuid;
use validator::Validate;

#[utoipa::path(
    put,
    path = "/category/updateCategory",
    tag = CATEGORY_TAG,
    request_body = UpdateCategoryDto,
    responses(
        (status = 200, description = "Category updated successfully", body = UpdateCategoryResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_category_api(
    State(state): State<AppState>,
    Json(payload): Json<UpdateCategoryDto>,
) -> AppResult<Json<UpdateCategoryResponseDto>> {
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let category = state
        .category_use_cases
        .update_category(UpdateCategoryCommand {
            id: Uuid::parse_str(payload.category_id.as_ref())
                .map_err(|_| AppError::BadRequest("Category id malformed".to_string()))?,
            name: payload.name,
            description: payload.description,
            color: payload.color,
        })
        .await?;

    Ok(Json(UpdateCategoryResponseDto {
        updated_category: CategoryDto {
            id: category.id.to_string(),
            name: category.name,
            description: category.description,
            color: category.color,
            tasks: Vec::new(), //TODO should return tasks?
        },
    }))
}
