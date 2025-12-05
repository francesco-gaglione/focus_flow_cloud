use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UpdateTaskData {
    category_id: Option<uuid::Uuid>,
    name: Option<String>,
    description: Option<String>,
    scheduled_date: Option<chrono::NaiveDate>,
    completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl UpdateTaskData {
    pub fn new(
        category_id: Option<uuid::Uuid>,
        name: Option<String>,
        description: Option<String>,
        scheduled_date: Option<chrono::NaiveDate>,
        completed_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        Self {
            category_id,
            name,
            description,
            scheduled_date,
            completed_at,
        }
    }

    pub fn with_category_id(mut self, category_id: Option<uuid::Uuid>) -> Self {
        self.category_id = category_id;
        self
    }

    pub fn with_name(mut self, name: Option<String>) -> Self {
        self.name = name;
        self
    }

    pub fn with_description(mut self, description: Option<String>) -> Self {
        self.description = description;
        self
    }

    pub fn with_scheduled_date(mut self, scheduled_date: Option<chrono::NaiveDate>) -> Self {
        self.scheduled_date = scheduled_date;
        self
    }

    pub fn with_completed_at(
        mut self,
        completed_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        self.completed_at = completed_at;
        self
    }

    pub fn category_id(&self) -> Option<Uuid> {
        self.category_id
    }

    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn scheduled_date(&self) -> Option<chrono::NaiveDate> {
        self.scheduled_date
    }

    pub fn completed_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.completed_at
    }
}
