use crate::adapters::http::dto::validators::validate_uuids::validate_uuids;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCategoriesDto {
    #[validate(custom(function = "validate_uuids"))]
    pub category_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteCategoriesResponseDto {
    pub deleted_ids: Vec<String>,
}
