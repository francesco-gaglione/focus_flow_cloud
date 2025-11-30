use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateTaskData {
    name: String,
    description: Option<String>,
    category_id: Option<Uuid>,
    scheduled_date: Option<NaiveDate>,
}

impl CreateTaskData {
    pub fn new(
        name: String,
        description: Option<String>,
        category_id: Option<Uuid>,
        scheduled_date: Option<NaiveDate>,
    ) -> Self {
        CreateTaskData {
            name,
            description,
            category_id,
            scheduled_date,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn category_id(&self) -> Option<&Uuid> {
        self.category_id.as_ref()
    }

    pub fn scheduled_date(&self) -> Option<&NaiveDate> {
        self.scheduled_date.as_ref()
    }
}
