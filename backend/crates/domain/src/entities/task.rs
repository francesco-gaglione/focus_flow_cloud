use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    id: Uuid,
    user_id: Uuid,
    category_id: Option<Uuid>,
    name: String,
    description: Option<String>,
    scheduled_date: Option<NaiveDate>,
    completed_at: Option<DateTime<Utc>>,
}

impl Task {
    pub fn reconstitute(
        id: Uuid,
        user_id: Uuid,
        category_id: Option<Uuid>,
        name: String,
        description: Option<String>,
        scheduled_date: Option<NaiveDate>,
        completed_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            user_id,
            category_id,
            name,
            description,
            scheduled_date,
            completed_at,
        }
    }

    pub fn create(
        user_id: Uuid,
        category_id: Option<Uuid>,
        name: String,
        description: Option<String>,
        scheduled_date: Option<NaiveDate>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            category_id,
            name,
            description,
            scheduled_date,
            completed_at: None,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn category_id(&self) -> Option<Uuid> {
        self.category_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn scheduled_date(&self) -> Option<NaiveDate> {
        self.scheduled_date
    }

    pub fn completed_at(&self) -> Option<DateTime<Utc>> {
        self.completed_at
    }

    pub fn is_completed(&self) -> bool {
        self.completed_at.is_some()
    }

    pub fn complete(&mut self) {
        self.completed_at = Some(Utc::now());
    }

    pub fn uncomplete(&mut self) {
        self.completed_at = None;
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn update_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    pub fn update_category(&mut self, category_id: Option<Uuid>) {
        self.category_id = category_id;
    }

    pub fn update_scheduled_date(&mut self, date: Option<NaiveDate>) {
        self.scheduled_date = date;
    }

    pub fn update_completed_at(&mut self, date: Option<DateTime<Utc>>) {
        self.completed_at = date;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use uuid::Uuid;

    #[test]
    fn test_task_creation() {
        let user_id = Uuid::new_v4();
        let name = "New Task".to_string();
        let task = Task::create(user_id, None, name.clone(), None, None);

        assert_eq!(task.user_id, user_id);
        assert_eq!(task.name, name);
        assert_eq!(task.completed_at, None);
        assert!(!task.is_completed());
    }

    #[test]
    fn test_task_completion_lifecycle() {
        let user_id = Uuid::new_v4();
        let mut task = Task::create(user_id, None, "Task".to_string(), None, None);

        task.complete();
        assert!(task.is_completed());
        assert!(task.completed_at.is_some());

        task.uncomplete();
        assert!(!task.is_completed());
        assert_eq!(task.completed_at, None);
    }

    #[test]
    fn test_task_updates() {
        let user_id = Uuid::new_v4();
        let mut task = Task::create(user_id, None, "Original Name".to_string(), None, None);

        let new_name = "Updated Name".to_string();
        task.update_name(new_name.clone());
        assert_eq!(task.name, new_name);

        let new_desc = Some("New Description".to_string());
        task.update_description(new_desc.clone());
        assert_eq!(task.description, new_desc);

        let new_category = Some(Uuid::new_v4());
        task.update_category(new_category);
        assert_eq!(task.category_id, new_category);

        let new_date = NaiveDate::from_ymd_opt(2023, 1, 1);
        task.update_scheduled_date(new_date);
        assert_eq!(task.scheduled_date, new_date);
    }
}
