use crate::adapters::http::app_state::AppState;
use crate::adapters::http::dto::category_api::create_category::{
    CreateCategoryDto, CreateCategoryResponseDto,
};
use crate::adapters::openapi::CATEGORY_TAG;
use crate::application::app_error::{AppError, AppResult};
use crate::application::use_cases::commands::create_category::CreateCategoryCommand;
use axum::Json;
use axum::extract::State;
use tracing::debug;
use validator::Validate;

#[utoipa::path(
    post,
    path = "/category/createCategory",
    tag = CATEGORY_TAG,
    request_body = CreateCategoryDto,
    responses(
        (status = 200, description = "Category created successfully", body = CreateCategoryResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 409, description = "Category already exists"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_category_api(
    State(state): State<AppState>,
    Json(payload): Json<CreateCategoryDto>,
) -> AppResult<Json<CreateCategoryResponseDto>> {
    debug!("{:?}", payload);
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    state
        .category_use_cases
        .create_category(&CreateCategoryCommand {
            name: payload.name,
            description: payload.description,
            color: payload.color,
        })
        .await?;

    Ok(Json(CreateCategoryResponseDto { created: true }))
}
