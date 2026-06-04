use std::sync::Arc;

use chrono::{DateTime, Utc};
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

use crate::repository_traits::{
    persistence_error::PersistenceError, reminder_persistence::ReminderPersistence,
};

#[derive(Debug, Error)]
pub enum GetPendingRemindersError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub struct PendingReminderOutput {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub date_time: DateTime<Utc>,
}

pub struct GetPendingRemindersUseCase {
    reminder_persistence: Arc<dyn ReminderPersistence>,
}

impl GetPendingRemindersUseCase {
    pub fn new(reminder_persistence: Arc<dyn ReminderPersistence>) -> Self {
        Self {
            reminder_persistence,
        }
    }

    #[instrument(skip(self))]
    pub async fn execute(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<PendingReminderOutput>, GetPendingRemindersError> {
        let reminders = self
            .reminder_persistence
            .find_pending_by_user(user_id)
            .await?;

        Ok(reminders
            .into_iter()
            .map(|r| PendingReminderOutput {
                id: r.id(),
                title: r.title().to_string(),
                description: r.description().to_string(),
                date_time: r.date(),
            })
            .collect())
    }
}
