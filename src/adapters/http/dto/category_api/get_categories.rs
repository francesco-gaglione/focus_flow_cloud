use crate::adapters::http::dto::common::category_dto::CategoryDto;
use crate::adapters::http::dto::common::task_dto::TaskDto;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetCategoriesResponseDto {
    pub categories: Vec<CategoryDto>,
    pub orphan_tasks: Vec<TaskDto>,
}
