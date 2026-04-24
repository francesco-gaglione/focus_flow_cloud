#[derive(Debug, Clone, PartialEq)]
pub struct Subtask {
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
}

impl Subtask {
    pub fn new(title: String, description: Option<String>) -> Self {
        Self {
            title,
            description,
            completed: false,
        }
    }

    pub fn update_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn update_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn mark_completed(&mut self) {
        self.completed = true;
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }
}

#[cfg(test)]
mod tests {
    use crate::entities::tasks::subtask::Subtask;

    #[test]
    fn test_task_creation() {
        let title = "Title";
        let description = "";
        let subtask = Subtask::new(title.to_string(), Some(description.to_string()));
        assert_eq!(subtask.title, title);
        assert_eq!(subtask.description, Some(description.to_string()));
    }

    #[test]
    fn test_task_getters() {
        let title = "Title";
        let description = "";
        let subtask = Subtask::new(title.to_string(), Some(description.to_string()));
        assert_eq!(subtask.title(), title);
        assert_eq!(subtask.description(), Some(description));
        assert!(!subtask.is_completed());
    }

    #[test]
    fn test_task_update() {
        let title = "Title";
        let description = "";
        let mut subtask = Subtask::new(title.to_string(), Some(description.to_string()));
        assert_eq!(subtask.title(), title);
        assert_eq!(subtask.description(), Some(description));
        subtask.update_title("New Title".to_string());
        assert_eq!(subtask.title(), "New Title");
        subtask.update_description("New Description".to_string());
        assert_eq!(subtask.description(), Some("New Description"));
    }

    #[test]
    fn test_task_completition() {
        let title = "Title";
        let description = "";
        let mut subtask = Subtask::new(title.to_string(), Some(description.to_string()));
        assert_eq!(subtask.title(), title);
        assert_eq!(subtask.description(), Some(description));
        assert!(!subtask.completed);
        subtask.mark_completed();
        assert!(subtask.completed);
    }
}
