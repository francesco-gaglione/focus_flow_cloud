use axum::{extract::State, Extension, Json};
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
pub struct DueFlashcardDto {
    pub id: Uuid,
    pub front: String,
    pub back: String,
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct DueFlashcardsResponseDto {
    pub flashcards: Vec<DueFlashcardDto>,
}

#[utoipa::path(
    get,
    path = "/api/flashcard/due",
    tag = FLASHCARD_TAG,
    summary = "Get all due (overdue) flashcards for the authenticated user",
    responses(
        (status = 200, description = "Due flashcards", body = DueFlashcardsResponseDto),
        (status = 401, description = "Unauthorized"),
    ),
    security(("jwt" = []))
)]
pub async fn get_due_flashcards_api(
    State(state): State<AppState>,
    Extension(user): Extension<UserSession>,
) -> HttpResult<Json<DueFlashcardsResponseDto>> {
    tracing::info!("Getting due flashcards for user_id={}", user.user_id);

    let cards = state
        .flashcards
        .get_due_flashcards_uc
        .execute(user.user_id)
        .await
        .map_err(|e| HttpError::GenericError(e.to_string()))?;

    let flashcards = cards
        .into_iter()
        .map(|c| DueFlashcardDto {
            id: *c.id(),
            front: c.front().to_string(),
            back: c.back().to_string(),
            due_date: c.due_date(),
        })
        .collect();

    Ok(Json(DueFlashcardsResponseDto { flashcards }))
}
