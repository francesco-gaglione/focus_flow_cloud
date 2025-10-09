#[derive(Debug, Clone)]
pub struct UpdateTaskData {
    pub category_id: Option<uuid::Uuid>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub scheduled_date: Option<chrono::NaiveDate>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}
