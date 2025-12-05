#[derive(Debug, Clone, PartialEq)]
pub struct CreateCategoryData {
    pub name: String,
    pub description: Option<String>,
    pub color: String,
}
