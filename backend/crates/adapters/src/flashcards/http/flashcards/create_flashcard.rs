use application::flashcards::use_cases::create_flashcards::{
    CreateFlashcardCommand, CreateFlashcardError,
};
use axum::{extract::State, Extension, Json};
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
pub struct CreateFlashcardDto {
    #[validate(length(
        min = 1,
        max = 500,
        message = "Card front side must be between 1 and 255 characters"
    ))]
    pub front: String,

    #[validate(length(
        min = 1,
        max = 500,
        message = "Back front side must be between 1 and 255 characters"
    ))]
    pub back: String,

    pub folder_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
pub struct CreateFlashcardResponseDto {}

impl From<CreateFlashcardError> for HttpError {
    fn from(value: CreateFlashcardError) -> Self {
        match value {
            CreateFlashcardError::PersistenceError(e) => {
                HttpError::GenericError("Unexpected error".to_string())
            }
            CreateFlashcardError::InvalidFrontSideCard => {
                HttpError::BadRequest("Invalid card front".to_string())
            }
            CreateFlashcardError::InvalidBackSideCard => {
                HttpError::BadRequest("Invalid card back".to_string())
            }
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/flashcard",
    tag = FLASHCARD_TAG,
    summary = "Create a new flashcard",
    request_body = CreateFlashcardDto,
    responses(
        (status = 201, description = "Flashcard created successfully", body = CreateFlashcardDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn create_flashcard_api(
    State(state): State<AppState>,
    Extension(user): Extension<UserSession>,
    Json(payload): Json<CreateFlashcardDto>,
) -> HttpResult<Json<CreateFlashcardResponseDto>> {
    payload
        .validate()
        .map_err(|e| HttpError::BadRequest(e.to_string()))?;
    tracing::info!("Creating flashcard");

    let command = CreateFlashcardCommand {
        front: payload.front,
        back: payload.back,
        user_id: user.user_id,
        folder_id: payload.folder_id,
    };

    let _ = state
        .flashcards
        .create_flashcard_uc
        .execute(command)
        .await?;

    tracing::info!("Flashcard created successfully");

    Ok(Json(CreateFlashcardResponseDto {}))
}
