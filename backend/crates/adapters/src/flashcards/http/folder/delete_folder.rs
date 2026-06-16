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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
pub struct DeleteFolderResponseDto {}

#[utoipa::path(
    delete,
    path = "/api/flashcard/folder/{id}",
    tag = FLASHCARD_TAG,
    summary = "Delete a folder and its contents",
    params(
        ("id" = Uuid, Path, description = "Folder ID")
    ),
    responses(
        (status = 200, description = "Folder deleted", body = DeleteFolderResponseDto),
        (status = 404, description = "Not found"),
        (status = 401, description = "Unauthorized"),
    ),
    security(("jwt" = []))
)]
pub async fn delete_folder_api(
    State(state): State<AppState>,
    Extension(_user): Extension<UserSession>,
    Path(id): Path<Uuid>,
) -> HttpResult<Json<DeleteFolderResponseDto>> {
    tracing::info!("Deleting folder id={}", id);

    state
        .flashcards
        .delete_folder_uc
        .execute(id)
        .await
        .map_err(|e| HttpError::GenericError(e.to_string()))?;

    Ok(Json(DeleteFolderResponseDto {}))
}
