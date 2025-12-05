use crate::adapters::http::dto::common::category_dto::CategoryDto;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetCategoryResponseDto {
    #[serde(flatten)]
    pub category: CategoryDto,
}
