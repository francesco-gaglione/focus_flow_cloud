use application::flashcards::use_cases::review_flashcard::{
    CardRatingCommand, ReviewFlashcardCommand, ReviewFlashcardError,
};
use axum::{
    extract::{Path, State},
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    http_error::{HttpError, HttpResult},
    openapi::FLASHCARD_TAG,
    shared::http::{app_state::AppState, model::session_model::UserSession},
};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
pub enum CardRatingDto {
    Again,
    Hard,
    Good,
    Easy,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct ReviewFlashcardDto {
    pub rating: CardRatingDto,
    pub elapsed_days: u32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
pub struct ReviewFlashcardResponseDto {}

impl From<ReviewFlashcardError> for HttpError {
    fn from(value: ReviewFlashcardError) -> Self {
        match value {
            ReviewFlashcardError::PersistenceError(e) => {
                use application::shared::traits::persistence_error::PersistenceError;
                match e {
                    PersistenceError::NotFound(msg) => HttpError::NotFound(msg),
                    _ => HttpError::GenericError("Unexpected error".to_string()),
                }
            }
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/flashcard/{id}/review",
    tag = FLASHCARD_TAG,
    summary = "Submit a review rating for a flashcard",
    params(
        ("id" = Uuid, Path, description = "Flashcard ID")
    ),
    request_body = ReviewFlashcardDto,
    responses(
        (status = 200, description = "Review recorded", body = ReviewFlashcardResponseDto),
        (status = 404, description = "Not found"),
        (status = 401, description = "Unauthorized"),
    ),
    security(("jwt" = []))
)]
pub async fn review_flashcard_api(
    State(state): State<AppState>,
    Extension(_user): Extension<UserSession>,
    Path(id): Path<Uuid>,
    Json(payload): Json<ReviewFlashcardDto>,
) -> HttpResult<Json<ReviewFlashcardResponseDto>> {
    tracing::info!("Reviewing flashcard id={} rating={:?}", id, payload.rating);

    let rating = match payload.rating {
        CardRatingDto::Again => CardRatingCommand::Again,
        CardRatingDto::Hard => CardRatingCommand::Hard,
        CardRatingDto::Good => CardRatingCommand::Good,
        CardRatingDto::Easy => CardRatingCommand::Easy,
    };

    let command = ReviewFlashcardCommand {
        flashcard_id: id,
        rating,
        elapsed_days: payload.elapsed_days,
    };

    state
        .flashcards
        .review_flashcard_uc
        .execute(command)
        .await?;

    Ok(Json(ReviewFlashcardResponseDto {}))
}
