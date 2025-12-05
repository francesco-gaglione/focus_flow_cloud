use crate::adapters::http::dto::validators::validate_uuid::validate_uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct DeleteCategoriesDto {
    #[validate(custom(function = "validate_uuid"))]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteCategoriesResponseDto {
    pub deleted_ids: Vec<String>,
}
