use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::adapters::http::dto::common::session_type_enum::SessionTypeEnum;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FocusSessionDto {
    pub id: String,
    pub category_id: Option<String>,
    pub task_id: Option<String>,
    pub session_type: SessionTypeEnum,
    pub actual_duration: Option<i64>,
    pub concentration_score: Option<i32>,
    pub notes: Option<String>,
    pub started_at: i64,
    pub ended_at: Option<i64>,
    pub created_at: i64,
}
