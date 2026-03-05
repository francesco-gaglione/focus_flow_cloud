use crate::http::app_state::AppState;
use crate::http::dto::common::category_dto::CategoryDto;
use crate::http_error::{map_persistence_error, HttpError, HttpResult};
use crate::openapi::CATEGORY_TAG;
use application::use_cases::category::get_category_usecase::GetCategoryError;
use axum::extract::{Path, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

impl From<GetCategoryError> for HttpError {
    fn from(value: GetCategoryError) -> Self {
        match value {
            GetCategoryError::PersistenceError(e) => map_persistence_error(e),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetCategoryResponseDto {
    #[serde(flatten)]
    pub category: CategoryDto,
}

#[utoipa::path(
    get,
    path = "/api/category/{id}",
    tag = CATEGORY_TAG,
    summary = "Get a category by id",
    params(
        ("id" = String, Path, description = "Category ID")
    ),
    responses(
        (status = 200, description = "Category", body = GetCategoryResponseDto),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Category not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn get_category(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> HttpResult<Json<GetCategoryResponseDto>> {
    let id = id
        .parse()
        .map_err(|_| HttpError::BadRequest("Invalid id".to_string()))?;
    let category = state.get_category_uc.execute(id).await?;

    let response = GetCategoryResponseDto {
        category: CategoryDto {
            id: category.id.to_string(),
            name: category.name.to_string(),
            description: category.description.map(|s| s.to_string()),
            color: category.color.to_string(),
            tasks: vec![],
        },
    };

    tracing::info!("Category retrieved successfully: {:?}", response);

    Ok(Json(response))
}
