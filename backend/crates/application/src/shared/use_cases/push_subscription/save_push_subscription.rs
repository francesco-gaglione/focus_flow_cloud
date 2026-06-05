use crate::{
    shared::traits::persistence_error::PersistenceError,
    shared::traits::push_subscription_persistence::PushSubscriptionPersistence,
};
use domain::tasks::entities::push_subscription::PushSubscription;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::traits::persistence_error::PersistenceError;
    use crate::shared::traits::push_subscription_persistence::MockPushSubscriptionPersistence;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_execute_success() {
        let mut mock = MockPushSubscriptionPersistence::new();
        let expected_id = Uuid::new_v4();

        mock.expect_upsert()
            .times(1)
            .returning(move |_| Ok(expected_id));

        let uc = SavePushSubscriptionUseCase::new(Arc::new(mock));
        let result = uc
            .execute(
                Uuid::new_v4(),
                "https://push.example.com/ep".to_string(),
                "p256dh".to_string(),
                "auth".to_string(),
            )
            .await;

        assert_eq!(result.unwrap(), expected_id);
    }

    #[tokio::test]
    async fn test_execute_persistence_error() {
        let mut mock = MockPushSubscriptionPersistence::new();

        mock.expect_upsert()
            .times(1)
            .returning(|_| Err(PersistenceError::Unexpected("db error".to_string())));

        let uc = SavePushSubscriptionUseCase::new(Arc::new(mock));
        let result = uc
            .execute(
                Uuid::new_v4(),
                "https://push.example.com/ep".to_string(),
                "p256dh".to_string(),
                "auth".to_string(),
            )
            .await;

        assert!(matches!(
            result.unwrap_err(),
            SavePushSubscriptionError::PersistenceError(_)
        ));
    }
}
