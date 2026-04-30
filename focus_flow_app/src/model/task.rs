use std::fmt::Display;

#[derive(Clone, PartialEq)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub cat: String,
    pub cat_color: String,
    pub priority: Option<TaskPriority>,
    pub due: TaskDue,
    pub completed_at: Option<chrono::NaiveDateTime>,
    pub done: bool,
}

#[derive(Clone, PartialEq)]
pub enum TaskDue {
    Overdue(chrono::NaiveDate),
    Today,
    Tomorrow,
    Upcoming(chrono::NaiveDate),
}

impl Display for TaskDue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskDue::Overdue(date) => write!(f, "Overdue: {}", date.format("%Y-%m-%d")),
            TaskDue::Today => write!(f, "Today"),
            TaskDue::Tomorrow => write!(f, "Tomorrow"),
            TaskDue::Upcoming(date) => write!(f, "Upcoming: {}", date.format("%Y-%m-%d")),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Urgent,
}
