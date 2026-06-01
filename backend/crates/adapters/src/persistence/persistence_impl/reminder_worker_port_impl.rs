use crate::workers::reminder_worker::process_reminder_job;
use apalis::layers::retry::RetryPolicy;
use apalis::prelude::*;
use apalis_postgres::{PgPool, PostgresStorage};
use apalis_sql::ext::TaskBuilderExt;
use application::repository_traits::push_subscription_persistence::PushSubscriptionPersistence;
use application::repository_traits::reminder_persistence::ReminderPersistence;
use application::repository_traits::reminder_worker_port::{
    ReminderWorkerPort, WorkerPortError, WorkerPortResult,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::Mutex;
use tracing::{error, info, instrument};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReminderJob {
    pub reminder_id: Uuid,
}

pub struct ReminderWorkerPortImpl {
    storage: Mutex<PostgresStorage<ReminderJob>>,
    pool: PgPool,
}

impl ReminderWorkerPortImpl {
    pub fn new(pool: PgPool) -> Self {
        Self {
            storage: Mutex::new(PostgresStorage::new(&pool)),
            pool,
        }
    }
}

#[async_trait]
impl ReminderWorkerPort for ReminderWorkerPortImpl {
    #[instrument(skip(self))]
    async fn schedule(&self, id: Uuid, date_time: DateTime<Utc>) -> WorkerPortResult<()> {
        let job = ReminderJob { reminder_id: id };
        let system_time =
            SystemTime::UNIX_EPOCH + Duration::from_secs(date_time.timestamp() as u64);

        info!("Scheduling reminder at {:?}", system_time);

        let task = Task::builder(job)
            .run_at_time(system_time)
            .max_attempts(5)
            .build();

        let mut storage = self.storage.lock().await;
        storage.push_task(task).await.map_err(|e| {
            error!("Failed to schedule reminder: {}", e);
            WorkerPortError::WorkerNotFound
        })?;

        Ok(())
    }

    #[instrument(skip(self))]
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

#[instrument(skip(pool, reminder_persistence, push_sub_persistence, vapid_private_key))]
pub async fn spawn_reminder_worker(
    pool: &PgPool,
    reminder_persistence: Arc<dyn ReminderPersistence>,
    push_sub_persistence: Arc<dyn PushSubscriptionPersistence>,
    vapid_private_key: String,
) {
    PostgresStorage::setup(pool).await.unwrap();
    let storage = PostgresStorage::new(pool);
    let worker = WorkerBuilder::new("reminder-worker")
        .backend(storage)
        .data(reminder_persistence)
        .data(push_sub_persistence)
        .data(vapid_private_key)
        .retry(RetryPolicy::retries(1))
        .build(process_reminder_job);

    tokio::spawn(async move {
        info!("Starting reminder worker");
        match worker.run().await {
            Ok(_) => info!("Reminder worker running"),
            Err(e) => error!("Failed to run reminder worker: {}", e),
        };
    });
}
