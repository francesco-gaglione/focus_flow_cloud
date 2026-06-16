use application::flashcards::use_cases::get_activity_heatmap::GetActivityHeatmapError;
use axum::{
    extract::{Query, State},
    Extension, Json,
};
use chrono::NaiveDate;
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
pub struct ActivityEntryDto {
    pub date: NaiveDate,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct ActivityHeatmapResponseDto {
    pub entries: Vec<ActivityEntryDto>,
}

#[derive(Deserialize)]
pub struct ActivityHeatmapQuery {
    pub days: Option<u32>,
}

impl From<GetActivityHeatmapError> for HttpError {
    fn from(e: GetActivityHeatmapError) -> Self {
        match e {
            GetActivityHeatmapError::PersistenceError(pe) => {
                HttpError::GenericError(pe.to_string())
            }
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/flashcard/stats/activity",
    tag = FLASHCARD_TAG,
    summary = "Get flashcard review activity heatmap",
    params(
        ("days" = Option<u32>, Query, description = "Number of days to look back (default 90)")
    ),
    responses(
        (status = 200, description = "Activity heatmap", body = ActivityHeatmapResponseDto),
        (status = 401, description = "Unauthorized"),
    ),
    security(("jwt" = []))
)]
pub async fn get_activity_heatmap_api(
    State(state): State<AppState>,
    Extension(user): Extension<UserSession>,
    Query(query): Query<ActivityHeatmapQuery>,
) -> HttpResult<Json<ActivityHeatmapResponseDto>> {
    let days = query.days.unwrap_or(90);

    let entries = state
        .flashcards
        .get_activity_heatmap_uc
        .execute(user.user_id, days)
        .await?;

    Ok(Json(ActivityHeatmapResponseDto {
        entries: entries
            .into_iter()
            .map(|e| ActivityEntryDto {
                date: e.date,
                count: e.count,
            })
            .collect(),
    }))
}
