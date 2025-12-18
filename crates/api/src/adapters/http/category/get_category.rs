use crate::adapters::http::app_state::AppState;
use crate::adapters::http::dto::common::category_dto::CategoryDto;
use crate::adapters::http_error::{HttpError, HttpResult};
use crate::adapters::openapi::CATEGORY_TAG;
use axum::extract::{Path, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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
    let category = state.get_category_usecase.execute(id).await?;

    let response = GetCategoryResponseDto {
        category: CategoryDto {
            id: category.id().to_string(),
            name: category.name().to_string(),
            description: category.description().map(|s| s.to_string()),
            color: category.color().to_string(),
            tasks: vec![], //TODO
        },
    };

    tracing::info!("Category retrieved successfully: {:?}", response);

    Ok(Json(response))
}
