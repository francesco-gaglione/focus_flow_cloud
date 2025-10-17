use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateConcentrationScore {
    #[validate(range(min = 0, max = 10))]
    pub concentration_score: i32,
}
