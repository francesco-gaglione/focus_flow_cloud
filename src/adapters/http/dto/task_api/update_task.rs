use crate::adapters::http::dto::common::task_dto::TaskDto;
use crate::adapters::http::dto::validators::validate_uuid::validate_uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTaskDto {
    #[validate(custom(function = "validate_uuid"))]
    pub task_id: String,

    #[validate(custom(function = "validate_uuid"))]
    pub category_id: Option<String>,

    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: Option<String>,

    #[validate(length(max = 255, message = "Description must not exceed 255 characters"))]
    pub description: Option<String>,

    pub scheduled_date: Option<i64>, //timestamp in seconds

    pub completed_at: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTaskResponseDto {
    pub updated_task: TaskDto,
}
