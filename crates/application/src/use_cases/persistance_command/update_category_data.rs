#[derive(Debug, Clone)]
pub struct UpdateCategoryData {
    pub name: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
}
