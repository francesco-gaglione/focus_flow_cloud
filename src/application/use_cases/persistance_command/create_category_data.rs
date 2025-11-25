#[derive(Debug, Clone)]
pub struct CreateCategoryData {
    pub name: String,
    pub description: Option<String>,
    pub color: String,
}
