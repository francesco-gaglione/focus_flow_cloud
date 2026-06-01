use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Reminder {
    id: Uuid,
    task_id: Option<Uuid>,
    user_id: Uuid,
    title: String,
    description: String,
    date: DateTime<Utc>,
    reminder_sent: bool,
}

impl Reminder {
    pub fn new(
        task_id: Option<Uuid>,
        user_id: Uuid,
        title: String,
        date: DateTime<Utc>,
        description: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            task_id,
            user_id,
            title,
            date,
            description,
            reminder_sent: false,
        }
    }

    pub fn reconstitute(
        id: Uuid,
        task_id: Option<Uuid>,
        user_id: Uuid,
        title: String,
        description: String,
        date: DateTime<Utc>,
        reminder_sent: bool,
    ) -> Self {
        Self {
            id,
            task_id,
            user_id,
            title,
            description,
            date,
            reminder_sent,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn task_id(&self) -> Option<Uuid> {
        self.task_id
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn mark_sent(&mut self) {
        self.reminder_sent = true;
    }

    pub fn is_sent(&self) -> bool {
        self.reminder_sent
    }

    pub fn is_overdue(&self) -> bool {
        self.date < Utc::now() && !self.reminder_sent
    }

    pub fn update_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn update_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn update_date(&mut self, date: DateTime<Utc>) {
        self.date = date;
    }

    pub fn date(&self) -> DateTime<Utc> {
        self.date
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    fn make_reminder(date: DateTime<Utc>) -> Reminder {
        Reminder::new(
            None,
            Uuid::new_v4(),
            "Test Reminder".to_string(),
            date,
            "Test description".to_string(),
        )
    }

    #[test]
    fn test_new_reminder() {
        let title = "Test Reminder".to_string();
        let date = Utc::now();
        let description = "Test description".to_string();
        let user_id = Uuid::new_v4();
        let reminder = Reminder::new(None, user_id, title.clone(), date, description.clone());
        assert_eq!(reminder.title, title);
        assert_eq!(reminder.date, date);
        assert_eq!(reminder.description, description);
        assert!(!reminder.reminder_sent);
        assert_eq!(reminder.task_id, None);
    }

    #[test]
    fn test_getters() {
        let date = Utc::now() + Duration::from_mins(10);
        let reminder = make_reminder(date);
        assert_eq!(reminder.title(), "Test Reminder");
        assert_eq!(reminder.date(), date);
        assert_eq!(reminder.description(), "Test description");
        assert!(!reminder.is_overdue());
    }

    #[test]
    fn test_update() {
        let date = Utc::now() + Duration::from_mins(20);
        let mut reminder = make_reminder(date);
        let new_title = "New Title".to_string();
        reminder.update_title(new_title.clone());
        assert_eq!(reminder.title(), new_title);
        let new_date = Utc::now() + Duration::from_mins(30);
        reminder.update_date(new_date);
        assert_eq!(reminder.date(), new_date);
        let new_description = "New description".to_string();
        reminder.update_description(new_description.clone());
        assert_eq!(reminder.description(), new_description);
        reminder.mark_sent();
        assert!(reminder.is_sent());
    }

    #[test]
    fn test_overdue() {
        let mut reminder = make_reminder(Utc::now() - Duration::from_mins(10));
        assert!(reminder.is_overdue());
        reminder.mark_sent();
        assert!(reminder.is_sent());
        assert!(!reminder.is_overdue());
    }

    #[test]
    fn test_task_id() {
        let task_id = Uuid::new_v4();
        let reminder = Reminder::new(
            Some(task_id),
            Uuid::new_v4(),
            "Title".to_string(),
            Utc::now(),
            "Desc".to_string(),
        );
        assert_eq!(reminder.task_id(), Some(task_id));
    }
}
