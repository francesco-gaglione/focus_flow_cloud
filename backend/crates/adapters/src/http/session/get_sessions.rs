use crate::http::app_state::AppState;
use crate::http::dto::validators::validate_uuids::validate_uuids;
use crate::http_error::{HttpError, HttpResult};
use crate::mappers::focus_session_mapper::FocusSessionMapper;
use crate::openapi::SESSION_TAG;
use application::use_cases::focus_session::command::find_session_filters::{
    ConcentrationScoreFilter, FindSessionFiltersCommand, FocusSessionDateFilter,
};
use axum::extract::{Query, State};
use axum::Json;
use domain::entities::focus_session_type::FocusSessionType;
use serde::{Deserialize, Serialize};
use tracing::debug;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

use crate::http::dto::common::{
    focus_session::FocusSessionDto, session_type_enum::SessionTypeEnum,
};

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct GetSessionFiltersDto {
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
    #[validate(custom(function = "validate_uuids"))]
    pub category_ids: Option<Vec<String>>,
    pub session_type: Option<SessionTypeEnum>,
    #[schema(example = "1")]
    pub min_concentration_score: Option<i32>,
    #[schema(example = "5")]
    pub max_concentration_score: Option<i32>,
    pub has_notes: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetSessionFiltersResponseDto {
    pub focus_sessions: Vec<FocusSessionDto>,
}

#[utoipa::path(
    get,
    path = "/api/focus-sessions",
    tag = SESSION_TAG,
    summary = "Get focus sessions by filters",
    params(
        GetSessionFiltersDto
    ),
    responses(
        (status = 200, description = "Sessions fetched successfully", body = GetSessionFiltersResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn get_sessions(
    State(state): State<AppState>,
    query: Query<GetSessionFiltersDto>,
) -> HttpResult<Json<GetSessionFiltersResponseDto>> {
    debug!("query: {:?}", query);

    if (query.start_date.is_some() && query.end_date.is_none())
        || (query.start_date.is_none() && query.end_date.is_some())
    {
        return Err(HttpError::BadRequest(
            "Both start and end date should be set, or none of them".to_string(),
        ));
    }

    if (query.min_concentration_score.is_some() && query.max_concentration_score.is_none())
        || (query.min_concentration_score.is_none() && query.max_concentration_score.is_some())
    {
        return Err(HttpError::BadRequest(
            "Both min and max concentration score should be set, or none of them".to_string(),
        ));
    }

    let date_range = match (query.start_date, query.end_date) {
        (Some(start), Some(end)) => Some(FocusSessionDateFilter {
            start_date: start,
            end_date: end,
        }),
        (None, None) => None,
        _ => return Err(HttpError::GenericError("Invalid state error".to_string())),
    };

    let concentration_score_range =
        match (query.min_concentration_score, query.max_concentration_score) {
            (Some(min), Some(max)) => Some(ConcentrationScoreFilter { min, max }),
            (None, None) => None,
            _ => unreachable!(),
        };

    let session_type = match &query.session_type {
        Some(t) => Some(
            FocusSessionType::from_str(t.as_str())
                .ok_or_else(|| HttpError::BadRequest("Invalid session type".to_string()))?,
        ),
        None => None,
    };

    let filters = FindSessionFiltersCommand {
        date_range,
        category_ids: query.category_ids.clone(),
        session_type,
        concentration_score_range,
        has_notes: query.has_notes,
    };

    let sessions = state
        .find_sessions_by_filters_usecase
        .execute(filters)
        .await?;

    let response_dto = GetSessionFiltersResponseDto {
        focus_sessions: sessions
            .into_iter()
            .map(|session| FocusSessionMapper::to_dto(&session))
            .collect(),
    };

    Ok(Json(response_dto))
}
