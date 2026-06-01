use crate::repository_traits::persistence_error::PersistenceResult;
use async_trait::async_trait;
use domain::entities::reminder::Reminder;
use uuid::Uuid;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ReminderPersistence: Send + Sync {
    async fn save_reminder(&self, reminder: Reminder) -> PersistenceResult<Uuid>;

    async fn get_reminder(&self, id: Uuid) -> PersistenceResult<Option<Reminder>>;

    async fn delete_reminder(&self, id: Uuid) -> PersistenceResult<()>;

    async fn find_by_task_ids(&self, task_ids: Vec<Uuid>) -> PersistenceResult<Vec<Reminder>>;
}
