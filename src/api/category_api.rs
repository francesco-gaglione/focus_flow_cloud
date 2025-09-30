use crate::AppState;
use crate::api::api_error::ApiError;
use crate::dto::create_category::{CreateCategoryDto, CreateCategoryResponseDto};
use crate::services::category_service::CreateCategoryCommand;
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde_json::{Value, json};
use validator::Validate;

async fn create_category_api(
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

pub fn router() -> Router<AppState> {
    Router::new().route("/createCategory", post(create_category_api))
}
