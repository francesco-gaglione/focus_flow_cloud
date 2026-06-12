use crate::shared::persistence::impls::reminder_worker_port_impl::ReminderJob;
use apalis::prelude::Data;
use application::shared::traits::push_subscription_persistence::PushSubscriptionPersistence;
use application::shared::traits::reminder_persistence::ReminderPersistence;
use domain::tasks::entities::push_subscription::PushSubscription;
use std::sync::Arc;
use tracing::{error, info, instrument, warn};
use web_push::{
    ContentEncoding, IsahcWebPushClient, SubscriptionInfo, VapidSignatureBuilder, WebPushClient,
    WebPushMessageBuilder,
};

async fn send_web_push(sub: &PushSubscription, title: &str, body: &str, vapid_private_key: String) {
    let endpoint = sub.endpoint().to_string();
    let p256dh = sub.p256dh().to_string();
    let auth = sub.auth().to_string();
    let title = title.to_string();
    let body = body.to_string();

    info!("Sending web push to endpoint: {}", endpoint);

    let result = tokio::task::spawn_blocking(move || {
        let subscription_info = SubscriptionInfo::new(&endpoint, &p256dh, &auth);

        let sig_builder = VapidSignatureBuilder::from_base64(
            &vapid_private_key,
            base64::URL_SAFE_NO_PAD,
            &subscription_info,
        )
        .map_err(|e| format!("VAPID build error: {:?}", e))?;

        let signature = sig_builder
            .build()
            .map_err(|e| format!("VAPID sign error: {:?}", e))?;

        let payload = serde_json::json!({ "title": title, "body": body }).to_string();

        let mut builder = WebPushMessageBuilder::new(&subscription_info);
        builder.set_payload(ContentEncoding::Aes128Gcm, payload.as_bytes());
        builder.set_vapid_signature(signature);

        let message = builder
            .build()
            .map_err(|e| format!("Message build error: {:?}", e))?;

        let client =
            IsahcWebPushClient::new().map_err(|e| format!("Client create error: {:?}", e))?;

        // isahc uses its own async executor — block_on is safe in spawn_blocking
        futures::executor::block_on(client.send(message))
            .map_err(|e| format!("Send error: {:?}", e))
    })
    .await;

    match result {
        Ok(Ok(())) => info!("Web push sent successfully"),
        Ok(Err(e)) => error!("Web push failed: {}", e),
        Err(e) => error!("spawn_blocking panicked: {:?}", e),
    }
}

#[instrument(skip_all)]
pub async fn process_reminder_job(
    job: ReminderJob,
    persistence: Data<Arc<dyn ReminderPersistence>>,
    push_sub_persistence: Data<Arc<dyn PushSubscriptionPersistence>>,
    vapid_private_key: Data<String>,
) {
    let reminder_id = job.reminder_id;
    info!("Processing reminder job for reminder {}", reminder_id);
    match persistence.get_reminder(reminder_id).await {
        Ok(Some(mut reminder)) => {
            reminder.mark_sent();
            if let Err(e) = persistence.update_reminder(reminder.clone()).await {
                error!("Failed to mark reminder {} as sent: {:?}", reminder_id, e);
                return;
            }
            info!("Reminder {} marked as sent", reminder_id);

            match push_sub_persistence
                .get_by_user_id(reminder.user_id())
                .await
            {
                Ok(subscriptions) => {
                    info!(
                        "Found {} push subscription(s) for user {}",
                        subscriptions.len(),
                        reminder.user_id()
                    );
                    for sub in subscriptions {
                        send_web_push(
                            &sub,
                            reminder.title(),
                            reminder.description(),
                            (*vapid_private_key).clone(),
                        )
                        .await;
                    }
                }
                Err(e) => {
                    error!(
                        "Failed to fetch push subscriptions for user {}: {:?}",
                        reminder.user_id(),
                        e
                    );
                }
            }
        }
        Ok(None) => {
            warn!("Reminder {} not found, skipping", reminder_id);
        }
        Err(e) => {
            error!("Failed to fetch reminder {}: {:?}", reminder_id, e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use application::shared::traits::persistence_error::PersistenceError;
    use application::shared::traits::persistence_error::PersistenceResult;
    use async_trait::async_trait;
    use domain::tasks::entities::push_subscription::PushSubscription;
    use domain::tasks::entities::reminder::Reminder;

    mockall::mock! {
        ReminderPersistenceMock {}
        #[async_trait]
        impl ReminderPersistence for ReminderPersistenceMock {
            async fn save_reminder(&self, reminder: Reminder) -> PersistenceResult<uuid::Uuid>;
            async fn get_reminder(&self, id: uuid::Uuid) -> PersistenceResult<Option<Reminder>>;
            async fn delete_reminder(&self, id: uuid::Uuid) -> PersistenceResult<()>;
            async fn find_by_task_ids(&self, task_ids: Vec<uuid::Uuid>) -> PersistenceResult<Vec<Reminder>>;
            async fn update_reminder(&self, reminder: Reminder) -> PersistenceResult<()>;
            async fn find_pending_by_user(&self, user_id: uuid::Uuid) -> PersistenceResult<Vec<Reminder>>;
        }
    }

    mockall::mock! {
        PushSubscriptionPersistenceMock {}
        #[async_trait]
        impl PushSubscriptionPersistence for PushSubscriptionPersistenceMock {
            async fn upsert(&self, subscription: PushSubscription) -> PersistenceResult<uuid::Uuid>;
            async fn get_by_user_id(&self, user_id: uuid::Uuid) -> PersistenceResult<Vec<PushSubscription>>;
            async fn delete(&self, id: uuid::Uuid) -> PersistenceResult<()>;
        }
    }
    use chrono::Utc;
    use std::sync::Arc;
    use uuid::Uuid;

    fn make_job(id: Uuid) -> ReminderJob {
        ReminderJob { reminder_id: id }
    }

    fn make_reminder(user_id: Uuid) -> Reminder {
        Reminder::new(
            None,
            user_id,
            "Test".to_string(),
            Utc::now(),
            "Body".to_string(),
        )
    }

    #[tokio::test]
    async fn test_reminder_not_found_skips() {
        let mut reminder_mock = MockReminderPersistenceMock::new();
        let push_mock = MockPushSubscriptionPersistenceMock::new();

        reminder_mock
            .expect_get_reminder()
            .times(1)
            .returning(|_| Ok(None));

        process_reminder_job(
            make_job(Uuid::new_v4()),
            Data::new(Arc::new(reminder_mock) as Arc<dyn ReminderPersistence>),
            Data::new(Arc::new(push_mock) as Arc<dyn PushSubscriptionPersistence>),
            Data::new("fake_key".to_string()),
        )
        .await;
    }

    #[tokio::test]
    async fn test_fetch_reminder_error_skips() {
        let mut reminder_mock = MockReminderPersistenceMock::new();
        let push_mock = MockPushSubscriptionPersistenceMock::new();

        reminder_mock
            .expect_get_reminder()
            .times(1)
            .returning(|_| Err(PersistenceError::Unexpected("db error".to_string())));

        process_reminder_job(
            make_job(Uuid::new_v4()),
            Data::new(Arc::new(reminder_mock) as Arc<dyn ReminderPersistence>),
            Data::new(Arc::new(push_mock) as Arc<dyn PushSubscriptionPersistence>),
            Data::new("fake_key".to_string()),
        )
        .await;
    }

    #[tokio::test]
    async fn test_update_reminder_fails_returns_early() {
        let mut reminder_mock = MockReminderPersistenceMock::new();
        let push_mock = MockPushSubscriptionPersistenceMock::new();
        let user_id = Uuid::new_v4();

        reminder_mock
            .expect_get_reminder()
            .times(1)
            .returning(move |_| Ok(Some(make_reminder(user_id))));

        reminder_mock
            .expect_update_reminder()
            .times(1)
            .returning(|_| Err(PersistenceError::Unexpected("update failed".to_string())));

        process_reminder_job(
            make_job(Uuid::new_v4()),
            Data::new(Arc::new(reminder_mock) as Arc<dyn ReminderPersistence>),
            Data::new(Arc::new(push_mock) as Arc<dyn PushSubscriptionPersistence>),
            Data::new("fake_key".to_string()),
        )
        .await;
    }

    #[tokio::test]
    async fn test_no_push_subscriptions() {
        let mut reminder_mock = MockReminderPersistenceMock::new();
        let mut push_mock = MockPushSubscriptionPersistenceMock::new();
        let user_id = Uuid::new_v4();

        reminder_mock
            .expect_get_reminder()
            .times(1)
            .returning(move |_| Ok(Some(make_reminder(user_id))));

        reminder_mock
            .expect_update_reminder()
            .times(1)
            .returning(|_| Ok(()));

        push_mock
            .expect_get_by_user_id()
            .times(1)
            .returning(|_| Ok(vec![]));

        process_reminder_job(
            make_job(Uuid::new_v4()),
            Data::new(Arc::new(reminder_mock) as Arc<dyn ReminderPersistence>),
            Data::new(Arc::new(push_mock) as Arc<dyn PushSubscriptionPersistence>),
            Data::new("fake_key".to_string()),
        )
        .await;
    }

    #[tokio::test]
    async fn test_push_subscription_fetch_fails() {
        let mut reminder_mock = MockReminderPersistenceMock::new();
        let mut push_mock = MockPushSubscriptionPersistenceMock::new();
        let user_id = Uuid::new_v4();

        reminder_mock
            .expect_get_reminder()
            .times(1)
            .returning(move |_| Ok(Some(make_reminder(user_id))));

        reminder_mock
            .expect_update_reminder()
            .times(1)
            .returning(|_| Ok(()));

        push_mock
            .expect_get_by_user_id()
            .times(1)
            .returning(|_| Err(PersistenceError::Unexpected("db error".to_string())));

        process_reminder_job(
            make_job(Uuid::new_v4()),
            Data::new(Arc::new(reminder_mock) as Arc<dyn ReminderPersistence>),
            Data::new(Arc::new(push_mock) as Arc<dyn PushSubscriptionPersistence>),
            Data::new("fake_key".to_string()),
        )
        .await;
    }
}
