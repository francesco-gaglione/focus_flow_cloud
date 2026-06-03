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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence::db_models::db_user::NewDbUser;
    use crate::persistence::persistence_impl::persistence::postgres_persistence;
    use crate::persistence::schema;
    use application::repository_traits::push_subscription_persistence::PushSubscriptionPersistence;
    use diesel::RunQueryDsl;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use domain::entities::push_subscription::PushSubscription;
    use testcontainers::runners::AsyncRunner;
    use testcontainers_modules::postgres::Postgres;
    use uuid::Uuid;

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../../migrations/");

    async fn setup() -> (
        crate::persistence::PostgresPersistence,
        testcontainers::ContainerAsync<Postgres>,
    ) {
        let container = Postgres::default().start().await.unwrap();
        let host = container.get_host().await.unwrap();
        let port = container.get_host_port_ipv4(5432).await.unwrap();
        let db_url = format!("postgres://postgres:postgres@{host}:{port}/postgres");

        let persistence = postgres_persistence(&db_url).await;

        let conn = persistence.pool.get().await.unwrap();
        conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()).unwrap())
            .await
            .unwrap();

        (persistence, container)
    }

    async fn create_test_user(persistence: &crate::persistence::PostgresPersistence) -> Uuid {
        let new_user = NewDbUser {
            username: format!("user_{}", Uuid::new_v4()),
            hashed_password: "hashed".to_string(),
            role: "user".to_string(),
        };
        let conn = persistence.pool.get().await.unwrap();
        conn.interact(move |conn| {
            diesel::insert_into(schema::users::table)
                .values(&new_user)
                .returning(schema::users::id)
                .get_result::<Uuid>(conn)
        })
        .await
        .unwrap()
        .unwrap()
    }

    fn make_subscription(user_id: Uuid) -> PushSubscription {
        PushSubscription::new(
            user_id,
            "https://push.example.com/endpoint".to_string(),
            "p256dh_key".to_string(),
            "auth_secret".to_string(),
        )
    }

    #[tokio::test]
    async fn test_upsert_inserts_new_subscription() {
        let (persistence, _container) = setup().await;
        let user_id = create_test_user(&persistence).await;

        let id = persistence
            .upsert(make_subscription(user_id))
            .await
            .unwrap();

        assert_ne!(id, Uuid::nil());
    }

    #[tokio::test]
    async fn test_upsert_updates_on_conflict() {
        let (persistence, _container) = setup().await;
        let user_id = create_test_user(&persistence).await;

        let sub = make_subscription(user_id);
        let id1 = persistence.upsert(sub.clone()).await.unwrap();

        // Same user_id + endpoint → conflict → update p256dh/auth
        let updated = PushSubscription::reconstitute(
            sub.id(),
            user_id,
            sub.endpoint().to_string(),
            "new_p256dh".to_string(),
            "new_auth".to_string(),
        );
        let id2 = persistence.upsert(updated).await.unwrap();

        // Same row updated, same id returned
        assert_eq!(id1, id2);

        let subs = persistence.get_by_user_id(user_id).await.unwrap();
        assert_eq!(subs.len(), 1);
        assert_eq!(subs[0].p256dh(), "new_p256dh");
        assert_eq!(subs[0].auth(), "new_auth");
    }

    #[tokio::test]
    async fn test_get_by_user_id_returns_empty_for_unknown_user() {
        let (persistence, _container) = setup().await;

        let result = persistence.get_by_user_id(Uuid::new_v4()).await.unwrap();
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_get_by_user_id_returns_all_subscriptions() {
        let (persistence, _container) = setup().await;
        let user_id = create_test_user(&persistence).await;

        let sub1 = PushSubscription::new(
            user_id,
            "https://push.example.com/ep1".to_string(),
            "p256dh_1".to_string(),
            "auth_1".to_string(),
        );
        let sub2 = PushSubscription::new(
            user_id,
            "https://push.example.com/ep2".to_string(),
            "p256dh_2".to_string(),
            "auth_2".to_string(),
        );

        persistence.upsert(sub1).await.unwrap();
        persistence.upsert(sub2).await.unwrap();

        let subs = persistence.get_by_user_id(user_id).await.unwrap();
        assert_eq!(subs.len(), 2);
    }

    #[tokio::test]
    async fn test_delete_removes_subscription() {
        let (persistence, _container) = setup().await;
        let user_id = create_test_user(&persistence).await;

        let id = persistence
            .upsert(make_subscription(user_id))
            .await
            .unwrap();

        persistence.delete(id).await.unwrap();

        let subs = persistence.get_by_user_id(user_id).await.unwrap();
        assert!(subs.is_empty());
    }

    #[tokio::test]
    async fn test_delete_nonexistent_is_ok() {
        let (persistence, _container) = setup().await;

        let result = persistence.delete(Uuid::new_v4()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_by_user_id_isolates_users() {
        let (persistence, _container) = setup().await;
        let user_a = create_test_user(&persistence).await;

        persistence.upsert(make_subscription(user_a)).await.unwrap();

        let subs_b = persistence.get_by_user_id(Uuid::new_v4()).await.unwrap();
        assert!(subs_b.is_empty());
    }
}
