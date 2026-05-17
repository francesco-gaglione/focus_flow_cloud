use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub struct Reminder {
    title: String,
    description: String,
    date: DateTime<Utc>,
    reminder_sent: bool,
}

impl Reminder {
    pub fn new(title: String, date: DateTime<Utc>, description: String) -> Self {
        Self {
            title,
            date,
            description,
            reminder_sent: false,
        }
    }

    pub fn mark_sent(&mut self) {
        self.reminder_sent = true;
    }

    pub fn is_sent(&self) -> bool {
        self.reminder_sent
    }

    // Returns true if the reminder is overdue (past due date and not sent)
    // Returns false if the reminder is not overdue or has already been sent
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

    #[test]
    fn test_new_reminder() {
        let title = "Test Reminder".to_string();
        let date = Utc::now();
        let description = "Test description".to_string();
        let reminder = Reminder::new(title.clone(), date, description.clone());
        assert_eq!(reminder.title, title);
        assert_eq!(reminder.date, date);
        assert_eq!(reminder.description, description);
        assert!(!reminder.reminder_sent);
    }

    // TODO: tests getters
    #[test]
    fn test_getters() {
        let title = "Test Reminder".to_string();
        let date = Utc::now() + Duration::from_mins(10);
        let description = "Test description".to_string();
        let reminder = Reminder::new(title.clone(), date, description.clone());
        assert_eq!(reminder.title(), title);
        assert_eq!(reminder.date(), date);
        assert_eq!(reminder.description(), description);
        assert!(!reminder.is_overdue());
    }

    #[test]
    fn test_update() {
        let title = "Old title".to_string();
        let date = Utc::now() + Duration::from_mins(20);
        let description = "Old description".to_string();
        let mut reminder = Reminder::new(title.clone(), date, description.clone());
        assert_eq!(reminder.title(), title);
        assert_eq!(reminder.date(), date);
        assert_eq!(reminder.description(), description);
        assert!(!reminder.is_overdue());
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
        let mut reminder = Reminder::new(
            "Overdue Title".to_string(),
            Utc::now() - Duration::from_mins(10),
            "Overdue description".to_string(),
        );
        assert!(reminder.is_overdue());
        reminder.mark_sent();
        assert!(reminder.is_sent());
        // After marking as sent, it should no longer be overdue
        assert!(!reminder.is_overdue());
    }
}
