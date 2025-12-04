use crate::adapters::http::dto::common::task_dto::TaskDto;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TasksResponseDto {
    pub tasks: Vec<TaskDto>,
}
