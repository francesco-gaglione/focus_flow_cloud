use crate::adapters::http::dto::session_api::session_type_enum::SessionTypeEnum;
use crate::adapters::http::dto::validators::validate_uuid::validate_uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateManualSessionDto {
    #[validate(custom(function = "validate_uuid"))]
    pub task_id: Option<String>,

    #[validate(custom(function = "validate_uuid"))]
    pub category_id: Option<String>,

    pub session_type: SessionTypeEnum,

    #[validate(range(min = 0, max = 10))]
    pub concentration_score: Option<i32>,

    //TODO validate
    pub started_at: i64, // timestamp in seconds

    pub ended_at: i64,

    //TODO should be validated?
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateManualSessionResponseDto {
    pub id: String,
}
