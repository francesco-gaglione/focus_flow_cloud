use crate::AppState;
use crate::dto::create_category::{CreateCategoryDto, CreateCategoryResponseDto};
use crate::services::category_service::CreateCategoryCommand;
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde_json::{Value, json};

async fn create_category_api(
    State(state): State<AppState>,
    Json(payload): Json<CreateCategoryDto>,
) -> Json<CreateCategoryResponseDto> {
    log::debug!("{:?}", payload);

    let category = state
        .category_service
        .create_category(CreateCategoryCommand {
            name: payload.name,
            description: payload.description,
            color: payload.color,
        })
        .await
        .map_err(|e| {}); //todo

    //TODO devo gestire gli errori e devo capire come si fa in axum in maniera corretta

    Json(CreateCategoryResponseDto {
        id: String::default(),
    })
}

pub fn router() -> Router<AppState> {
    Router::new().route("/category", post(create_category_api))
}
