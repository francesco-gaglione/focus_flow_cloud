use crate::http::app_state::AppState;
use crate::http::dto::common::category_dto::CategoryDto;
use crate::http::dto::validators::validate_uuid::validate_uuid;
use crate::http_error::{HttpError, HttpResult};
use crate::openapi::CATEGORY_TAG;
use application::use_cases::category::command::update_category::UpdateCategoryCommand;
use axum::extract::{Path, State};
use axum::Json;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

lazy_static! {
    static ref COLOR_REGEX: Regex = Regex::new(r"^#[0-9A-Fa-f]{6}$").unwrap();
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateCategoryPathDto {
    #[validate(custom(function = "validate_uuid"))]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCategoryDto {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: Option<String>,

    #[validate(length(max = 255, message = "Description must not exceed 255 characters"))]
    pub description: Option<String>,

    #[validate(regex(path = *COLOR_REGEX, message = "Color must be a valid hex code"))]
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCategoryResponseDto {
    pub updated_category: CategoryDto,
}

#[utoipa::path(
    put,
    path = "/api/category/{id}",
    tag = CATEGORY_TAG,
    summary = "Update a category",
    params(
        ("id" = String, Path, description = "Category ID to update")
    ),
    request_body = UpdateCategoryDto,
    security(
        ("jwt" = [])
    ),
    responses(
        (status = 200, description = "Category updated successfully", body = UpdateCategoryResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 401, description = "Unauthorized"),
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
