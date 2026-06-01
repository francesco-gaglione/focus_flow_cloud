use apalis_core::backend::TaskSink;
use apalis_core::task::builder::TaskBuilder;
use apalis_postgres::{PgPool, PostgresStorage};
use application::repository_traits::reminder_worker_port::{
    ReminderWorkerPort, WorkerPortError, WorkerPortResult,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx;
use std::time::{Duration, SystemTime};
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ReminderJob {
    reminder_id: Uuid,
}

pub struct ReminderWorkerPortImpl {
    storage: Mutex<PostgresStorage<ReminderJob>>,
    pool: PgPool,
}

impl ReminderWorkerPortImpl {
    pub fn new(pool: PgPool) -> Self {
        Self {
            storage: Mutex::new(PostgresStorage::new(&pool)),
            pool: pool.clone(),
        }
    }

    pub async fn setup(&self) -> Result<(), sqlx::Error> {
        PostgresStorage::<(), (), ()>::setup(&self.pool).await
    }
}

#[async_trait]
impl ReminderWorkerPort for ReminderWorkerPortImpl {
    async fn schedule(&self, id: Uuid, date_time: DateTime<Utc>) -> WorkerPortResult<()> {
        let job = ReminderJob { reminder_id: id };
        let system_time =
            SystemTime::UNIX_EPOCH + Duration::from_secs(date_time.timestamp() as u64);

        let task = TaskBuilder::new(job).run_at_time(system_time).build();

        let mut storage = self.storage.lock().await;
        TaskSink::push_task(&mut *storage, task)
            .await
            .map_err(|_| WorkerPortError::WorkerNotFound)?;

        Ok(())
    }

    async fn unschedule(&self, id: Uuid) -> WorkerPortResult<()> {
        sqlx::query(
            "DELETE FROM apalis.jobs \
             WHERE job_type = 'reminder' \
             AND convert_from(job, 'UTF8')::jsonb->>'reminder_id' = $1 \
             AND status = 'Pending'",
        )
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|_| WorkerPortError::WorkerNotFound)?;

        Ok(())
    }
}
