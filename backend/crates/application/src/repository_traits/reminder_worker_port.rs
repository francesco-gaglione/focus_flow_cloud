use async_trait::async_trait;
use chrono::{DateTime, Utc};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error, PartialEq)]
pub enum WorkerPortError {
    #[error("worker not found")]
    WorkerNotFound,
}

pub type WorkerPortResult<T> = Result<T, WorkerPortError>;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ReminderWorkerPort: Send + Sync {
    async fn schedule(&self, id: Uuid, date_time: DateTime<Utc>) -> WorkerPortResult<()>;

    async fn unschedule(&self, id: Uuid) -> WorkerPortResult<()>;
}
