use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompleteSession {
    #[validate(range(min = 0, max = 10))]
    pub concentration_score: Option<i32>,

    pub actual_duration: i64, // seconds
}
