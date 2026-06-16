use application::flashcards::use_cases::get_global_stats::GetGlobalStatsError;
use axum::{extract::State, Extension, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    http_error::{HttpError, HttpResult},
    openapi::FLASHCARD_TAG,
    shared::http::{app_state::AppState, model::session_model::UserSession},
};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct FlashcardGlobalStatsDto {
    pub total_cards: i64,
    pub due_today: i64,
    pub reviewed_today: i64,
    pub retention_rate_30d: f64,
}

impl From<GetGlobalStatsError> for HttpError {
    fn from(e: GetGlobalStatsError) -> Self {
        match e {
            GetGlobalStatsError::PersistenceError(pe) => HttpError::GenericError(pe.to_string()),
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/flashcard/stats",
    tag = FLASHCARD_TAG,
    summary = "Get global flashcard statistics for the authenticated user",
    responses(
        (status = 200, description = "Global stats", body = FlashcardGlobalStatsDto),
        (status = 401, description = "Unauthorized"),
    ),
    security(("jwt" = []))
)]
pub async fn get_flashcard_stats_api(
    State(state): State<AppState>,
    Extension(user): Extension<UserSession>,
) -> HttpResult<Json<FlashcardGlobalStatsDto>> {
    let stats = state
        .flashcards
        .get_global_stats_uc
        .execute(user.user_id)
        .await?;

    Ok(Json(FlashcardGlobalStatsDto {
        total_cards: stats.total_cards,
        due_today: stats.due_today,
        reviewed_today: stats.reviewed_today,
        retention_rate_30d: stats.retention_rate_30d,
    }))
}
