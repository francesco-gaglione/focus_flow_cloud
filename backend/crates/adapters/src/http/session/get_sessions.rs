use crate::http::app_state::AppState;
use crate::http::dto::validators::validate_uuids::validate_uuids;
use crate::http_error::{HttpError, HttpResult};
use crate::openapi::SESSION_TAG;
use application::use_cases::focus_session::find_sessions_by_filters::{
    ConcentrationScoreFilter, FindSessionByFiltersError, FindSessionFiltersCommand,
    FocusSessionDateFilter, FocusSessionOutput,
};

impl From<FindSessionByFiltersError> for HttpError {
    fn from(value: FindSessionByFiltersError) -> Self {
        match value {
            FindSessionByFiltersError::InvalidDateRange(e) => HttpError::BadRequest(e),
            FindSessionByFiltersError::InvalidCategoryId => {
                HttpError::BadRequest("Invalid category id".to_string())
            }
            FindSessionByFiltersError::InvalidTaskId => {
                HttpError::BadRequest("Invalid task id".to_string())
            }
            FindSessionByFiltersError::PersistenceError(e) => {
                HttpError::GenericError(e.to_string())
            }
        }
    }
}
use axum::extract::{Extension, Query, State};
use axum::Json;
use domain::entities::focus_session_type::FocusSessionType;
use serde::{Deserialize, Deserializer, Serialize};
use tracing::debug;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

use crate::http::dto::common::{
    focus_session::FocusSessionDto, session_type_enum::SessionTypeEnum,
};
use crate::http::model::session_model::UserSession;

fn deserialize_option_string_or_vec<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrVec {
        String(String),
        Vec(Vec<String>),
    }

    let value = Option::<StringOrVec>::deserialize(deserializer)?;

    match value {
        Some(StringOrVec::Vec(v)) => Ok(Some(v)),
        Some(StringOrVec::String(s)) => {
            if s.is_empty() {
                Ok(None)
            } else {
                Ok(Some(s.split(',').map(|s| s.trim().to_string()).collect()))
            }
        }
        None => Ok(None),
    }
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct GetSessionFiltersDto {
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_string_or_vec")]
    #[validate(custom(function = "validate_uuids"))]
    pub category_ids: Option<Vec<String>>,
    #[serde(default, deserialize_with = "deserialize_option_string_or_vec")]
    #[validate(custom(function = "validate_uuids"))]
    pub task_ids: Option<Vec<String>>,
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

impl From<FocusSessionOutput> for FocusSessionDto {
    fn from(value: FocusSessionOutput) -> Self {
        Self {
            id: value.id.to_string(),
            category_id: value.category_id.map(|id| id.to_string()),
            task_id: value.task_id.map(|id| id.to_string()),
            session_type: value.session_type.into(),
            actual_duration: value.actual_duration,
            concentration_score: value.concentration_score,
            notes: value.notes,
            started_at: value.started_at.timestamp(),
            ended_at: value.ended_at.map(|e| e.timestamp()),
            created_at: value.created_at.timestamp(),
        }
    }
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
    Extension(session): Extension<UserSession>,
    query: Query<GetSessionFiltersDto>,
) -> HttpResult<Json<GetSessionFiltersResponseDto>> {
    debug!("query: {:?}", query);
    let state = state;

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
        user_id: session.user_id,
        date_range,
        category_ids: query.category_ids.clone(),
        task_ids: query.task_ids.clone(),
        session_type,
        concentration_score_range,
        has_notes: query.has_notes,
    };

    let sessions = state
        .find_sessions_by_filters_usecase
        .execute(filters)
        .await?;

    let response_dto = GetSessionFiltersResponseDto {
        focus_sessions: sessions.into_iter().map(|session| session.into()).collect(),
    };

    Ok(Json(response_dto))
}
