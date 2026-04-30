#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TaskDue {
    Overdue(String),
    Today,
    Tomorrow,
    Upcoming(String),
}

impl TaskDue {
    pub fn label(&self) -> &str {
        match self {
            TaskDue::Overdue(s) => s.as_str(),
            TaskDue::Today => "TODAY",
            TaskDue::Tomorrow => "TOMORROW",
            TaskDue::Upcoming(s) => s.as_str(),
        }
    }

    pub fn css_class(&self) -> &str {
        match self {
            TaskDue::Overdue(_) => "todo-due overdue",
            TaskDue::Today => "todo-due today",
            _ => "todo-due",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub cat: String,
    pub cat_color: String,
    pub due: TaskDue,
    pub done: bool,
}
