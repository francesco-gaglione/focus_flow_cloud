use application::flashcards::use_cases::update_flashcard::{
    UpdateFlashcardCommand, UpdateFlashcardError,
};
use axum::{
    extract::{Path, State},
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    http_error::{HttpError, HttpResult},
    openapi::FLASHCARD_TAG,
    shared::http::{app_state::AppState, model::session_model::UserSession},
};

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct UpdateFlashcardDto {
    #[validate(length(
        min = 1,
        max = 500,
        message = "Card front side must be between 1 and 500 characters"
    ))]
    pub front: Option<String>,

    #[validate(length(
        min = 1,
        max = 500,
        message = "Card back side must be between 1 and 500 characters"
    ))]
    pub back: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
pub struct UpdateFlashcardResponseDto {}

impl From<UpdateFlashcardError> for HttpError {
    fn from(value: UpdateFlashcardError) -> Self {
        match value {
            UpdateFlashcardError::PersistenceError(e) => {
                use application::shared::traits::persistence_error::PersistenceError;
                match e {
                    PersistenceError::NotFound(msg) => HttpError::NotFound(msg),
                    _ => HttpError::GenericError("Unexpected error".to_string()),
                }
            }
            UpdateFlashcardError::NothingToDo => {
                HttpError::BadRequest("Nothing to update".to_string())
            }
        }
    }
}

#[utoipa::path(
    put,
    path = "/api/flashcard/{id}",
    tag = FLASHCARD_TAG,
    summary = "Update a flashcard",
    params(
        ("id" = Uuid, Path, description = "Flashcard ID")
    ),
    request_body = UpdateFlashcardDto,
    responses(
        (status = 200, description = "Flashcard updated", body = UpdateFlashcardResponseDto),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Not found"),
        (status = 401, description = "Unauthorized"),
    ),
    security(("jwt" = []))
)]
pub async fn update_flashcard_api(
    State(state): State<AppState>,
    Extension(_user): Extension<UserSession>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateFlashcardDto>,
) -> HttpResult<Json<UpdateFlashcardResponseDto>> {
    payload
        .validate()
        .map_err(|e| HttpError::BadRequest(e.to_string()))?;
    tracing::info!("Updating flashcard id={}", id);

    let command = UpdateFlashcardCommand {
        card_id: id,
        front: payload.front,
        back: payload.back,
    };

    state
        .flashcards
        .update_flashcard_uc
        .execute(command)
        .await?;

    Ok(Json(UpdateFlashcardResponseDto {}))
}
