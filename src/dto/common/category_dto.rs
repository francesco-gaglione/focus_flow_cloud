use crate::dto::common::task_dto::TaskDto;
use crate::entities::category::Category;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CategoryDto {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub tasks: Vec<TaskDto>,
}

impl From<Category> for CategoryDto {
    fn from(value: Category) -> Self {
        Self {
            id: value.id.to_string(),
            name: value.name.clone(),
            description: value.description.clone(),
            color: value.color,
            tasks: value.tasks.iter().map(|t| t.into()).collect(),
        }
    }
}
