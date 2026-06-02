use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::session_type_enum::SessionTypeEnum;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct FocusSessionDto {
    pub id: String,
    pub task_id: Option<String>,
    pub session_type: SessionTypeEnum,
    #[cfg_attr(feature = "ts", ts(type = "number | null"))]
    pub actual_duration: Option<i64>,
    pub concentration_score: Option<i32>,
    pub notes: Option<String>,
    #[cfg_attr(feature = "ts", ts(type = "number"))]
    pub started_at: i64,
    #[cfg_attr(feature = "ts", ts(type = "number | null"))]
    pub ended_at: Option<i64>,
}
