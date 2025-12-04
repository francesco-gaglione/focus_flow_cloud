use crate::adapters::http::dto::validators::validate_uuid::validate_uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateFocusSessionPathDto {
    #[validate(custom(function = "validate_uuid"))]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFocusSessionDto {
    #[validate(custom(function = "validate_uuid"))]
    pub category_id: Option<String>,

    #[validate(custom(function = "validate_uuid"))]
    pub task_id: Option<String>,

    #[validate(range(min = 0, max = 5))]
    pub concentration_score: Option<i32>,

    //TODO validate
    pub started_at: Option<i64>, // timestamp in seconds

    pub ended_at: Option<i64>,

    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateFocusSessionResponseDto {}
