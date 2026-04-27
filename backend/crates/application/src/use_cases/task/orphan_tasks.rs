use crate::repository_traits::{
    persistence_error::PersistenceError, task_persistence::TaskPersistence,
};
use chrono::{DateTime, Utc};
use domain::entities::tasks::task::Task;
use domain::entities::tasks::task_priority::TaskPriority;
use std::sync::Arc;
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum OrphanTasksError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

#[derive(Debug)]
pub struct GetOrphanTasksCommand {
    pub completed: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct TaskOutput {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<TaskPriority>,
    pub due_date: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl From<&Task> for TaskOutput {
    fn from(value: &Task) -> Self {
        Self {
            id: value.id(),
            user_id: value.user_id(),
            title: value.title().to_string(),
            description: value.description().map(|d| d.to_string()),
            priority: value.priority(),
            due_date: value.due_date(),
            completed_at: value.completed_at(),
        }
    }
}

pub struct OrphanTasksUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl OrphanTasksUseCase {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    #[instrument(skip(self))]
    pub async fn execute(
        &self,
        command: GetOrphanTasksCommand,
    ) -> Result<Vec<TaskOutput>, OrphanTasksError> {
        let res = self
            .task_persistence
            .find_all(command.completed.unwrap_or(false))
            .await?;
        Ok(res.iter().map(|t| t.into()).collect())
    }
}
