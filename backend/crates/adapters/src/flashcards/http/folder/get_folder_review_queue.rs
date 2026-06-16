use application::flashcards::use_cases::get_folder_review_queue::GetFolderReviewQueueError;
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
pub struct FolderDueFlashcardDto {
    pub id: Uuid,
    pub front: String,
    pub back: String,
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct FolderReviewQueueResponseDto {
    pub flashcards: Vec<FolderDueFlashcardDto>,
}

impl From<GetFolderReviewQueueError> for HttpError {
    fn from(e: GetFolderReviewQueueError) -> Self {
        match e {
            GetFolderReviewQueueError::PersistenceError(pe) => {
                HttpError::GenericError(pe.to_string())
            }
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/flashcard/folder/{folder_id}/review/queue",
    tag = FLASHCARD_TAG,
    summary = "Get due flashcards for review in a specific folder",
    params(
        ("folder_id" = Uuid, Path, description = "Folder ID")
    ),
    responses(
        (status = 200, description = "Due flashcards for folder", body = FolderReviewQueueResponseDto),
        (status = 401, description = "Unauthorized"),
    ),
    security(("jwt" = []))
)]
pub async fn get_folder_review_queue_api(
    State(state): State<AppState>,
    Extension(_user): Extension<UserSession>,
    Path(folder_id): Path<Uuid>,
) -> HttpResult<Json<FolderReviewQueueResponseDto>> {
    let cards = state
        .flashcards
        .get_folder_review_queue_uc
        .execute(folder_id)
        .await?;

    Ok(Json(FolderReviewQueueResponseDto {
        flashcards: cards
            .into_iter()
            .map(|c| FolderDueFlashcardDto {
                id: *c.id(),
                front: c.front().to_string(),
                back: c.back().to_string(),
                due_date: c.due_date(),
            })
            .collect(),
    }))
}
