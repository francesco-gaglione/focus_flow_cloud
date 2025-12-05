use crate::adapters::http::dto::validators::validate_uuids::validate_uuids;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

use crate::adapters::http::dto::common::{
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
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetSessionFiltersResponseDto {
    pub focus_sessions: Vec<FocusSessionDto>,
}
