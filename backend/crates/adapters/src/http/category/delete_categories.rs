use crate::http::app_state::AppState;
use crate::http::validators::validate_uuid::validate_uuid;
use crate::http_error::{HttpError, HttpResult};
use crate::openapi::CATEGORY_TAG;
use application::tasks::use_cases::category::delete_categories_usecase::DeleteCategoriesError;
use axum::extract::{Path, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::{debug, error};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
pub struct DeleteCategoriesDto {
    #[validate(custom(function = "validate_uuid"))]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
pub struct DeleteCategoriesResponseDto {
    pub deleted_ids: Vec<String>,
}

impl From<DeleteCategoriesError> for HttpError {
    fn from(value: DeleteCategoriesError) -> Self {
        match value {
            DeleteCategoriesError::PersistenceError(_) => {
                HttpError::GenericError("Generic Error".to_string())
            }
        }
    }
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

    let res = state
        .delete_categories_uc
        .execute(vec![category_id])
        .await?;

    debug!("Deleted {} categories", res.len());

    Ok(Json(DeleteCategoriesResponseDto {
        deleted_ids: res.iter().map(|id| id.to_string()).collect(),
    }))
}
