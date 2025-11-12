use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePomodoroContext {
    category_id: Option<String>,
    task_id: Option<String>,
}

impl UpdatePomodoroContext {
    pub fn new(category_id: Option<String>, task_id: Option<String>) -> Self {
        Self {
            category_id,
            task_id,
        }
    }

    pub fn category_id(&self) -> Option<String> {
        self.category_id.clone()
    }

    pub fn task_id(&self) -> Option<String> {
        self.task_id.clone()
    }
}

