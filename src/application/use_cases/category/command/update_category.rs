#[derive(Debug, Clone)]
pub struct UpdateCategoryCommand {
    pub id: uuid::Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
}
