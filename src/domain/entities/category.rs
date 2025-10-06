use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
}
