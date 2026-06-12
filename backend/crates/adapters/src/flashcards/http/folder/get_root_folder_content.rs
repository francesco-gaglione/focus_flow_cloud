use axum::{extract::State, Extension, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    flashcards::http::folder::get_folder_content::{build_response, FlashcardDto, FolderDto},
    http_error::HttpResult,
    openapi::FLASHCARD_TAG,
    shared::http::{app_state::AppState, model::session_model::UserSession},
};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct RootFolderContentsResponseDto {
    pub folder_id: Uuid,
    pub folders: Vec<FolderDto>,
    pub flashcards: Vec<FlashcardDto>,
}

#[utoipa::path(
    get,
    path = "/api/flashcard/folder/root/contents",
    tag = FLASHCARD_TAG,
    summary = "Get root folder contents for the authenticated user",
    responses(
        (status = 200, description = "Root folder contents", body = RootFolderContentsResponseDto),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(("jwt" = []))
)]
pub async fn get_root_folder_contents_api(
    State(state): State<AppState>,
    Extension(user): Extension<UserSession>,
) -> HttpResult<Json<RootFolderContentsResponseDto>> {
    tracing::info!("Getting root folder contents for user_id={}", user.user_id);

    let (root_id, contents) = state
        .flashcards
        .get_folder_contents_uc
        .execute_for_root(user.user_id)
        .await?;

    let inner = build_response(contents);
    Ok(Json(RootFolderContentsResponseDto {
        folder_id: root_id,
        folders: inner.folders,
        flashcards: inner.flashcards,
    }))
}
