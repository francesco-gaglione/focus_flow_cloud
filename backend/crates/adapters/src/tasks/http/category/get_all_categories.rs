use crate::shared::http::app_state::AppState;
use crate::tasks::http::dto::category_dto::CategoryDto;
use crate::http_error::{map_persistence_error, HttpError, HttpResult};
use crate::openapi::CATEGORY_TAG;
use application::tasks::use_cases::category::get_all_category_usecase::GetAllCategoryError;
use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

impl From<GetAllCategoryError> for HttpError {
    fn from(value: GetAllCategoryError) -> Self {
        match value {
            GetAllCategoryError::PersistenceError(e) => map_persistence_error(e),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct GetAllCategoryResponseDto {
    pub categories: Vec<CategoryDto>,
}

#[utoipa::path(
    get,
    path = "/api/category/categories",
    tag = CATEGORY_TAG,
    summary = "Get all categories",
    responses(
        (status = 200, description = "Category", body = GetAllCategoryResponseDto),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Category not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn get_all_categories_api(
    State(state): State<AppState>,
) -> HttpResult<Json<GetAllCategoryResponseDto>> {
    let categories = state.tasks.get_all_category_uc.execute().await?;

    let response = GetAllCategoryResponseDto {
        categories: categories
            .iter()
            .map(|c| CategoryDto {
                id: c.id.to_string(),
                name: c.name.to_string(),
                color: c.color.to_string(),
            })
            .collect(),
    };

    tracing::info!("Categories retrieved successfully.");

    Ok(Json(response))
}
