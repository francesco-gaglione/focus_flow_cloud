use application::flashcards::use_cases::get_folder_stats::GetFolderStatsError;
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
#[serde(rename_all = "camelCase")]
pub struct FlashcardFolderStatsDto {
    pub total_cards: i64,
    pub due_today: i64,
    pub reviewed_today: i64,
}

impl From<GetFolderStatsError> for HttpError {
    fn from(e: GetFolderStatsError) -> Self {
        match e {
            GetFolderStatsError::PersistenceError(pe) => HttpError::GenericError(pe.to_string()),
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/flashcard/folder/{folder_id}/stats",
    tag = FLASHCARD_TAG,
    summary = "Get flashcard statistics for a specific folder",
    params(
        ("folder_id" = Uuid, Path, description = "Folder ID")
    ),
    responses(
        (status = 200, description = "Folder stats", body = FlashcardFolderStatsDto),
        (status = 401, description = "Unauthorized"),
    ),
    security(("jwt" = []))
)]
pub async fn get_folder_stats_api(
    State(state): State<AppState>,
    Extension(user): Extension<UserSession>,
    Path(folder_id): Path<Uuid>,
) -> HttpResult<Json<FlashcardFolderStatsDto>> {
    let stats = state
        .flashcards
        .get_folder_stats_uc
        .execute(folder_id, user.user_id)
        .await?;

    Ok(Json(FlashcardFolderStatsDto {
        total_cards: stats.total_cards,
        due_today: stats.due_today,
        reviewed_today: stats.reviewed_today,
    }))
}
