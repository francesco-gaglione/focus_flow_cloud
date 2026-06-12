use crate::http::app_state::AppState;
use crate::http::validators::validate_uuid::validate_uuid;
use crate::http_error::{map_persistence_error, HttpError, HttpResult};
use crate::openapi::CATEGORY_TAG;
use application::tasks::use_cases::category::update_category_usecase::{
    UpdateCategoryCommand, UpdateCategoryError,
};

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct UpdateCategoryDto {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct UpdateCategoryResponseDto {
    pub success: bool,
}

impl From<UpdateCategoryError> for HttpError {
    fn from(value: UpdateCategoryError) -> Self {
        match value {
            UpdateCategoryError::PersistenceError(e) => map_persistence_error(e),
        }
    }
}
use axum::extract::{Path, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateCategoryPathDto {
    #[validate(custom(function = "validate_uuid"))]
    pub id: String,
}

#[utoipa::path(
    patch,
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

    state
        .update_category_uc
        .execute(UpdateCategoryCommand {
            id: category_id,
            name: payload.name,
            color: payload.color,
        })
        .await?;

    Ok(Json(UpdateCategoryResponseDto { success: true }))
}
