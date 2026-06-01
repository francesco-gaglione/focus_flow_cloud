use crate::persistence::db_models::db_push_subscription::{
    DbPushSubscription, NewDbPushSubscription,
};
use crate::persistence::schema;
use crate::persistence::PostgresPersistence;
use application::repository_traits::persistence_error::{PersistenceError, PersistenceResult};
use application::repository_traits::push_subscription_persistence::PushSubscriptionPersistence;
use async_trait::async_trait;
use chrono::Utc;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use domain::entities::push_subscription::PushSubscription;
use tracing::instrument;
use uuid::Uuid;

#[async_trait]
impl PushSubscriptionPersistence for PostgresPersistence {
    #[instrument(skip(self))]
    async fn upsert(&self, subscription: PushSubscription) -> PersistenceResult<Uuid> {
        let new_sub = NewDbPushSubscription {
            user_id: subscription.user_id(),
            endpoint: subscription.endpoint().to_string(),
            p256dh: subscription.p256dh().to_string(),
            auth: subscription.auth().to_string(),
            created_at: Utc::now(),
        };

        self.with_transaction(move |conn| {
            diesel::insert_into(schema::push_subscriptions::table)
                .values(&new_sub)
                .on_conflict((
                    schema::push_subscriptions::user_id,
                    schema::push_subscriptions::endpoint,
                ))
                .do_update()
                .set((
                    schema::push_subscriptions::p256dh
                        .eq(diesel::upsert::excluded(schema::push_subscriptions::p256dh)),
                    schema::push_subscriptions::auth
                        .eq(diesel::upsert::excluded(schema::push_subscriptions::auth)),
                ))
                .returning(schema::push_subscriptions::id)
                .get_result(conn)
        })
        .await
    }

    #[instrument(skip(self))]
    async fn get_by_user_id(&self, user_id: Uuid) -> PersistenceResult<Vec<PushSubscription>> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let results = conn
            .interact(move |conn| {
                schema::push_subscriptions::table
                    .filter(schema::push_subscriptions::user_id.eq(user_id))
                    .select(DbPushSubscription::as_select())
                    .load(conn)
            })
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
            .map_err(|e: diesel::result::Error| PersistenceError::Unexpected(e.to_string()))?;

        Ok(results.into_iter().map(Into::into).collect())
    }

    #[instrument(skip(self))]
    async fn delete(&self, id: Uuid) -> PersistenceResult<()> {
        self.with_transaction(move |conn| {
            diesel::delete(
                schema::push_subscriptions::table.filter(schema::push_subscriptions::id.eq(id)),
            )
            .execute(conn)
            .map(|_| ())
        })
        .await
    }
}
