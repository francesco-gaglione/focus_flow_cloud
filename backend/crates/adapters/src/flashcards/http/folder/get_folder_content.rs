use application::flashcards::use_cases::get_folder_contents::GetFolderContentsError;
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
pub struct FolderDto {
    pub id: Uuid,
    pub name: String,
    pub parent_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct FlashcardDto {
    pub id: Uuid,
    pub front: String,
    pub back: String,
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct FolderContentsResponseDto {
    pub folders: Vec<FolderDto>,
    pub flashcards: Vec<FlashcardDto>,
}

pub fn build_response(
    contents: application::flashcards::use_cases::get_folder_contents::FolderContents,
) -> FolderContentsResponseDto {
    FolderContentsResponseDto {
        folders: contents
            .folders
            .into_iter()
            .map(|f| FolderDto {
                id: f.id(),
                name: f.name().to_string(),
                parent_id: f.parent_id(),
            })
            .collect(),
        flashcards: contents
            .flashcards
            .into_iter()
            .map(|f| FlashcardDto {
                id: *f.id(),
                front: f.front().to_string(),
                back: f.back().to_string(),
                due_date: f.due_date(),
            })
            .collect(),
    }
}

impl From<GetFolderContentsError> for HttpError {
    fn from(e: GetFolderContentsError) -> Self {
        match e {
            GetFolderContentsError::PersistenceError(_) => {
                HttpError::GenericError("Unexpected error".to_string())
            }
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/flashcard/folder/{folder_id}/contents",
    tag = FLASHCARD_TAG,
    summary = "Get folder contents (sub-folders and flashcards)",
    params(
        ("folder_id" = Uuid, Path, description = "Folder ID")
    ),
    responses(
        (status = 200, description = "Folder contents", body = FolderContentsResponseDto),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(("jwt" = []))
)]
pub async fn get_folder_contents_api(
    State(state): State<AppState>,
    Extension(_user): Extension<UserSession>,
    Path(folder_id): Path<Uuid>,
) -> HttpResult<Json<FolderContentsResponseDto>> {
    tracing::info!("Getting folder contents for folder_id={}", folder_id);

    let contents = state
        .flashcards
        .get_folder_contents_uc
        .execute(folder_id)
        .await?;

    Ok(Json(build_response(contents)))
}
