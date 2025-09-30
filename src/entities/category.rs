use crate::db_models::db_category::DbCategory;
use crate::entities::task::Task;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub tasks: Vec<Task>,
}

impl From<&DbCategory> for Category {
    fn from(value: &DbCategory) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            description: value.description.clone(),
            color: value.color.clone(),
            tasks: vec![],
        }
    }
}
