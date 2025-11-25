use crate::adapters::http::app_state::AppState;
use crate::adapters::http::dto::category_api::create_category::{
    CreateCategoryDto, CreateCategoryResponseDto,
};
use crate::adapters::http_error::{HttpError, HttpResult};
use crate::adapters::openapi::CATEGORY_TAG;
use crate::application::use_cases::category::command::create_category::CreateCategoryCommand;
use axum::extract::State;
use axum::Json;
use tracing::debug;
use validator::Validate;

#[utoipa::path(
    post,
    path = "/api/categories",
    tag = CATEGORY_TAG,
    summary = "Create a new category",
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
) -> HttpResult<Json<CreateCategoryResponseDto>> {
    debug!("{:?}", payload);
    payload
        .validate()
        .map_err(|e| HttpError::BadRequest(e.to_string()))?;

    state
        .create_category_usecase
        .execute(CreateCategoryCommand {
            name: payload.name,
            description: payload.description,
            color: payload.color,
        })
        .await?;

    Ok(Json(CreateCategoryResponseDto { created: true }))
}
