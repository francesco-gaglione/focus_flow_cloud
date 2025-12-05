#[derive(Debug, Clone)]
pub struct CreateCategoryCommand {
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
}
