use crate::adapters::http::dto::common::session_type_enum::SessionTypeEnum;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StartSessionDto {
    pub session_type: SessionTypeEnum,
    pub start_date: i64,
    pub category_id: Option<String>,
    pub task_id: Option<String>,
}
