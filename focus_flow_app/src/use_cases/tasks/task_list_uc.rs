use chrono::{DateTime, Local, Utc};
use dioxus::logger::tracing::debug;
use shared::task::{TaskPriority, TasksResponseDto};

use crate::clients::{
    category_http_client::get_all_categories, http_client::ApiError,
    task_http_client::get_all_tasks,
};

#[derive(Debug, thiserror::Error)]
pub enum TaskListError {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error("Api error: {0}")]
    ApiError(#[from] ApiError),
}

pub type TaskListResult<T> = Result<T, TaskListError>;

pub struct TaskList {
    pub tasks: Vec<TodoTask>,
    pub categories: Vec<TodoCategory>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TodoCategory {
    pub name: String,
    pub id: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TodoSubtask {
    pub id: String,
    pub title: String,
    pub is_completed: bool,
    pub sort_order: i16,
}

#[derive(Clone, PartialEq)]
pub struct TodoTask {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub cat: Option<String>,
    pub cat_id: Option<String>,
    pub cat_color: Option<String>,
    pub priority: Option<TaskPriority>,
    pub due: TaskDue,
    pub due_date_set: bool,
    pub completed_at: Option<chrono::NaiveDateTime>,
    pub done: bool,
    pub subtasks: Vec<TodoSubtask>,
}

#[derive(Clone, PartialEq)]
pub enum TaskDue {
    Overdue(chrono::NaiveDate),
    Today(chrono::NaiveDate),
    Tomorrow(chrono::NaiveDate),
    Upcoming(chrono::NaiveDate),
}

impl std::fmt::Display for TaskDue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskDue::Overdue(date) => write!(f, "Overdue ({})", date.format("%b %d")),
            TaskDue::Today(_) => write!(f, "Today"),
            TaskDue::Tomorrow(_) => write!(f, "Tomorrow"),
            TaskDue::Upcoming(date) => write!(f, "{}", date.format("%b %d")),
        }
    }
}

//Fetch all tasks and related info for todo page
pub async fn task_list_uc(category_id: Option<&str>) -> TaskListResult<TaskList> {
    let tasks_dto: TasksResponseDto = get_all_tasks().await?;
    debug!("Tasks dto: {:?}", tasks_dto);
    let categories = get_all_categories().await?;

    let mut todo_tasks: Vec<TodoTask> = vec![];

    for task in tasks_dto.tasks {
        let category = if let Some(cat_id) = task.category_id {
            categories
                .categories
                .iter()
                .find(|c| c.id == cat_id.to_string())
        } else {
            None
        };

        let today = Local::now().date_naive();
        let tomorrow = today.succ_opt().unwrap_or(today);

        let due_date_set = task.due_date.is_some();
        let due = match task.due_date {
            Some(ts) => {
                let date = DateTime::from_timestamp(ts, 0)
                    .map(|dt: DateTime<Utc>| dt.with_timezone(&Local).date_naive())
                    .unwrap_or(today);
                if date < today {
                    TaskDue::Overdue(date)
                } else if date == today {
                    TaskDue::Today(date)
                } else if date == tomorrow {
                    TaskDue::Tomorrow(date)
                } else {
                    TaskDue::Upcoming(date)
                }
            }
            None => TaskDue::Upcoming(today),
        };

        let subtasks = task
            .subtasks
            .into_iter()
            .map(|s| TodoSubtask {
                id: s.id,
                title: s.title,
                is_completed: s.is_completed,
                sort_order: s.sort_order,
            })
            .collect();

        todo_tasks.push(TodoTask {
            id: task.id,
            title: task.title,
            description: task.description,
            cat: if category.is_some() {
                Some(category.unwrap().name.clone())
            } else {
                None
            },
            cat_id: task.category_id.map(|c| c.to_string()),
            cat_color: if category.is_some() {
                Some(category.unwrap().color.clone())
            } else {
                None
            },
            priority: task.priority,
            due,
            due_date_set,
            completed_at: task.completed_at.and_then(|ts| {
                DateTime::from_timestamp(ts, 0)
                    .map(|dt: DateTime<Utc>| dt.with_timezone(&Local).naive_local())
            }),
            done: task.completed_at.is_some(),
            subtasks: subtasks,
        });
    }

    Ok(TaskList {
        tasks: todo_tasks,
        categories: categories
            .categories
            .iter()
            .map(|c| TodoCategory {
                name: c.name.clone(),
                id: c.id.clone(),
            })
            .collect(),
    })
}
