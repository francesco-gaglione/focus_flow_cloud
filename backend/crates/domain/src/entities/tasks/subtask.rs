use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Subtask {
    id: Uuid,
    title: String,
    description: Option<String>,
    completed: bool,
    sort_order: i16,
}

impl Subtask {
    pub fn new(
        title: String,
        sort_order: i16,
        description: Option<String>,
        id: Option<Uuid>,
    ) -> Self {
        Self {
            id: id.unwrap_or_else(Uuid::new_v4),
            title,
            description,
            completed: false,
            sort_order,
        }
    }

    pub fn update_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn update_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn update_sort_order(&mut self, sort_order: i16) {
        self.sort_order = sort_order;
    }

    pub fn mark_completed(&mut self) {
        self.completed = true;
    }

    pub fn mark_incomplete(&mut self) {
        self.completed = false;
    }

    pub fn id(&self) -> Uuid {
        self.id
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

    pub fn sort_order(&self) -> i16 {
        self.sort_order
    }
}

#[cfg(test)]
mod tests {
    use crate::entities::tasks::subtask::Subtask;

    #[test]
    fn test_task_creation() {
        let title = "Title";
        let description = "";
        let subtask = Subtask::new(title.to_string(), 0, Some(description.to_string()), None);
        assert_eq!(subtask.title, title);
        assert_eq!(subtask.description, Some(description.to_string()));
        assert_eq!(subtask.sort_order, 0);
    }

    #[test]
    fn test_task_getters() {
        let title = "Title";
        let description = "";
        let subtask = Subtask::new(title.to_string(), 0, Some(description.to_string()), None);
        assert_eq!(subtask.title(), title);
        assert_eq!(subtask.description(), Some(description));
        assert!(!subtask.is_completed());
    }

    #[test]
    fn test_task_update() {
        let title = "Title";
        let description = "";
        let mut subtask = Subtask::new(title.to_string(), 0, Some(description.to_string()), None);
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
        let mut subtask = Subtask::new(title.to_string(), 0, Some(description.to_string()), None);
        assert_eq!(subtask.title(), title);
        assert_eq!(subtask.description(), Some(description));
        assert!(!subtask.completed);
        subtask.mark_completed();
        assert!(subtask.completed);
    }
}
