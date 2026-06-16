use axum::{
    extract::{Path, State},
    Extension, Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    http_error::{HttpError, HttpResult},
    openapi::FLASHCARD_TAG,
    shared::http::{app_state::AppState, model::session_model::UserSession},
};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct GetFlashcardResponseDto {
    pub id: Uuid,
    pub front: String,
    pub back: String,
    pub due_date: Option<DateTime<Utc>>,
}

#[utoipa::path(
    get,
    path = "/api/flashcard/{id}",
    tag = FLASHCARD_TAG,
    summary = "Get a flashcard by ID",
    params(
        ("id" = Uuid, Path, description = "Flashcard ID")
    ),
    responses(
        (status = 200, description = "Flashcard found", body = GetFlashcardResponseDto),
        (status = 404, description = "Not found"),
        (status = 401, description = "Unauthorized"),
    ),
    security(("jwt" = []))
)]
pub async fn get_flashcard_api(
    State(state): State<AppState>,
    Extension(_user): Extension<UserSession>,
    Path(id): Path<Uuid>,
) -> HttpResult<Json<GetFlashcardResponseDto>> {
    tracing::info!("Getting flashcard id={}", id);

    let card = state
        .flashcards
        .get_flashcard_uc
        .execute(id)
        .await
        .map_err(|e| HttpError::GenericError(e.to_string()))?;

    Ok(Json(GetFlashcardResponseDto {
        id: *card.id(),
        front: card.front().to_string(),
        back: card.back().to_string(),
        due_date: card.due_date(),
    }))
}
