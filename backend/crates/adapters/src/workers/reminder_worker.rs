use crate::persistence::persistence_impl::reminder_worker_port_impl::ReminderJob;
use apalis::prelude::Data;
use application::repository_traits::push_subscription_persistence::PushSubscriptionPersistence;
use application::repository_traits::reminder_persistence::ReminderPersistence;
use domain::entities::push_subscription::PushSubscription;
use std::sync::Arc;
use tracing::{error, info, instrument, warn};
use web_push::{
    ContentEncoding, IsahcWebPushClient, SubscriptionInfo, VapidSignatureBuilder, WebPushClient,
    WebPushMessageBuilder,
};

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
