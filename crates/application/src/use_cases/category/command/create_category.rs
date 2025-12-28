pub struct CreateCategoryCommand {
    pub user_id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
}
