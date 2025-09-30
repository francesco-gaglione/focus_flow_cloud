use crate::AppState;
use crate::api::api_error::ApiError;
use crate::dto::category_api::create_category::{CreateCategoryDto, CreateCategoryResponseDto};
use crate::services::category_service::CreateCategoryCommand;
use axum::Json;
use axum::extract::State;
use validator::Validate;

#[utoipa::path(
    post,
    path = "/category/createCategory",
    tag = "category",
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
) -> Result<Json<CreateCategoryResponseDto>, ApiError> {
    log::debug!("{:?}", payload);
    payload.validate()?;

    let category = state
        .category_service
        .create_category(CreateCategoryCommand {
            name: payload.name,
            description: payload.description,
            color: payload.color,
        })
        .await?;

    Ok(Json(CreateCategoryResponseDto {
        id: category.id.to_string(),
    }))
}
