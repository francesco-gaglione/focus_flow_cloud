use crate::adapters::http::app_state::AppState;
use crate::adapters::http_error::{HttpError, HttpResult};
use crate::adapters::openapi::CATEGORY_TAG;
use application::use_cases::category::command::create_category::CreateCategoryCommand;
use axum::extract::State;
use axum::Json;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tracing::debug;
use utoipa::ToSchema;
use validator::Validate;

lazy_static! {
    static ref COLOR_REGEX: Regex = Regex::new(r"^#[0-9A-Fa-f]{6}$").unwrap();
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateCategoryDto {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(max = 255))]
    pub description: Option<String>,
    #[validate(regex(path = *COLOR_REGEX, message = "Color must be a valid hex code"))]
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateCategoryResponseDto {
    pub category_id: String,
}

#[utoipa::path(
    post,
    path = "/api/category",
    tag = CATEGORY_TAG,
    summary = "Create a new category",
    request_body = CreateCategoryDto,
    responses(
        (status = 200, description = "Category created successfully", body = CreateCategoryResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 409, description = "Category already exists"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
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

    let category_id = state
        .create_category_usecase
        .execute(CreateCategoryCommand {
            name: payload.name,
            description: payload.description,
            color: payload.color,
        })
        .await?;

    Ok(Json(CreateCategoryResponseDto {
        category_id: category_id.to_string(),
    }))
}
