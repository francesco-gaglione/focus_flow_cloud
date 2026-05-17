use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct TaskDto {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<TaskPriority>,
    pub schedule: TaskScheduleDto,
    pub completed_at: Option<i64>,
    pub subtasks: Vec<SubtaskDto>,
    pub category_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct SubtaskDto {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub is_completed: bool,
    pub sort_order: i16,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Urgent,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub enum TaskScheduleDto {
    Unscheduled,
    AllDay { date: i64 },
    At { starts_at: i64 },
    Span { starts_at: i64, duration: i64 },
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct CreateSubtaskDto {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Title must be between 1 and 255 characters"
    ))]
    pub title: String,
    #[validate(length(max = 255, message = "Description must not exceed 255 characters"))]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskDto {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Title must be between 1 and 255 characters"
    ))]
    pub title: String,
    #[validate(length(max = 255, message = "Description must not exceed 255 characters"))]
    pub description: Option<String>,
    pub schedule: Option<TaskScheduleDto>,
    #[validate(nested)]
    pub subtasks: Option<Vec<CreateSubtaskDto>>,
    pub category_id: Option<Uuid>,
    pub priority: Option<TaskPriority>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct CreateTaskResponseDto {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct UpdateTaskDto {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Title must be between 1 and 255 characters"
    ))]
    pub title: Option<String>,
    #[validate(length(max = 255, message = "Description must not exceed 255 characters"))]
    pub description: Option<String>,
    pub schedule: Option<TaskScheduleDto>,
    pub completed: Option<bool>,
    pub priority: Option<TaskPriority>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct UpdateTaskResponseDto {
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct TasksResponseDto {
    pub tasks: Vec<TaskDto>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct UpdateSubTaskDto {
    pub completed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct UpdateSubTaskResponseDto {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteTaskDto {
    pub task_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct DeleteTaskResponseDto {
    pub deleted_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct CreateSubtaskResponseDto {
    pub id: String,
}
