use async_trait::async_trait;
use domain::tasks::entities::push_subscription::PushSubscription;
use uuid::Uuid;

use super::persistence_error::PersistenceResult;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PushSubscriptionPersistence: Send + Sync {
    async fn upsert(&self, subscription: PushSubscription) -> PersistenceResult<Uuid>;
    async fn get_by_user_id(&self, user_id: Uuid) -> PersistenceResult<Vec<PushSubscription>>;
    async fn delete(&self, id: Uuid) -> PersistenceResult<()>;
}
