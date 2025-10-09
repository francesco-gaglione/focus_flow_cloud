use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateTaskData {
    pub name: String,
    pub description: Option<String>,
    pub category_id: Option<Uuid>,
    pub scheduled_date: Option<NaiveDate>,
}
