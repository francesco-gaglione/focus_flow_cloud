use crate::repository_traits::persistence_error::PersistenceError;
use crate::repository_traits::push_subscription_persistence::PushSubscriptionPersistence;
use domain::entities::push_subscription::PushSubscription;
use std::sync::Arc;
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug, Error, PartialEq)]
pub enum SavePushSubscriptionError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type SavePushSubscriptionResult<T> = Result<T, SavePushSubscriptionError>;

pub struct SavePushSubscriptionUseCase {
    persistence: Arc<dyn PushSubscriptionPersistence>,
}

impl SavePushSubscriptionUseCase {
    pub fn new(persistence: Arc<dyn PushSubscriptionPersistence>) -> Self {
        Self { persistence }
    }

    #[instrument(skip(self))]
    pub async fn execute(
        &self,
        user_id: Uuid,
        endpoint: String,
        p256dh: String,
        auth: String,
    ) -> SavePushSubscriptionResult<Uuid> {
        let sub = PushSubscription::new(user_id, endpoint, p256dh, auth);
        Ok(self.persistence.upsert(sub).await?)
    }
}
