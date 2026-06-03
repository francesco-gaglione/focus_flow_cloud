use crate::persistence::db_models::db_reminder::UpdateDbReminder;
use crate::persistence::db_models::db_reminder::{DbReminder, NewDbReminder};
use crate::persistence::schema;
use crate::persistence::PostgresPersistence;
use application::repository_traits::persistence_error::{PersistenceError, PersistenceResult};
use application::repository_traits::reminder_persistence::ReminderPersistence;
use async_trait::async_trait;
use diesel::{
    ExpressionMethods, NullableExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl,
    SelectableHelper,
};
use domain::entities::reminder::Reminder;
use tracing::instrument;
use uuid::Uuid;

#[async_trait]
impl ReminderPersistence for PostgresPersistence {
    #[instrument(skip(self))]
    async fn save_reminder(&self, reminder: Reminder) -> PersistenceResult<Uuid> {
        let new_db_reminder = NewDbReminder::from(reminder);
        let id = new_db_reminder.id;

        self.with_transaction(move |conn| {
            diesel::insert_into(schema::reminders::table)
                .values(&new_db_reminder)
                .execute(conn)?;
            Ok(id)
        })
        .await
    }

    #[instrument(skip(self))]
    async fn get_reminder(&self, id: Uuid) -> PersistenceResult<Option<Reminder>> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let result = conn
            .interact(move |conn| {
                schema::reminders::table
                    .filter(schema::reminders::id.eq(id))
                    .select(DbReminder::as_select())
                    .first(conn)
                    .optional()
            })
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
            .map_err(|e: diesel::result::Error| PersistenceError::Unexpected(e.to_string()))?;

        Ok(result.map(|r| r.into()))
    }

    #[instrument(skip(self))]
    async fn delete_reminder(&self, id: Uuid) -> PersistenceResult<()> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        conn.interact(move |conn| {
            diesel::delete(schema::reminders::table.filter(schema::reminders::id.eq(id)))
                .execute(conn)
        })
        .await
        .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
        .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        Ok(())
    }

    #[instrument(skip(self))]
    async fn find_by_task_ids(&self, task_ids: Vec<Uuid>) -> PersistenceResult<Vec<Reminder>> {
        if task_ids.is_empty() {
            return Ok(vec![]);
        }

        self.with_transaction(move |conn| {
            let db_reminders: Vec<DbReminder> = schema::reminders::table
                .filter(
                    schema::reminders::task_id
                        .assume_not_null()
                        .eq_any(&task_ids),
                )
                .select(DbReminder::as_select())
                .load(conn)?;

            Ok(db_reminders.into_iter().map(|r| r.into()).collect())
        })
        .await
    }

    #[instrument(skip(self))]
    async fn update_reminder(&self, reminder: Reminder) -> PersistenceResult<()> {
        let id = reminder.id();
        let update = UpdateDbReminder {
            title: Some(reminder.title().to_string()),
            description: Some(reminder.description().to_string()),
            date: Some(reminder.date()),
            reminder_sent: Some(reminder.is_sent()),
        };

        self.with_transaction(move |conn| {
            diesel::update(schema::reminders::table.filter(schema::reminders::id.eq(id)))
                .set(&update)
                .execute(conn)?;
            Ok(())
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use crate::persistence::db_models::db_user::NewDbUser;
    use crate::persistence::persistence_impl::persistence::postgres_persistence;
    use crate::persistence::schema;
    use application::repository_traits::reminder_persistence::ReminderPersistence;
    use chrono::Utc;
    use diesel::RunQueryDsl;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use domain::entities::reminder::Reminder;
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

    fn make_reminder(user_id: Uuid) -> Reminder {
        Reminder::new(
            None,
            user_id,
            "Test reminder".to_string(),
            Utc::now() + chrono::Duration::hours(1),
            "Test description".to_string(),
        )
    }

    #[tokio::test]
    async fn test_save_and_get_reminder() {
        let (persistence, _container) = setup().await;
        let user_id = create_test_user(&persistence).await;
        let reminder = make_reminder(user_id);
        let reminder_id = reminder.id();

        persistence.save_reminder(reminder).await.unwrap();

        let found = persistence.get_reminder(reminder_id).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().id(), reminder_id);
    }

    #[tokio::test]
    async fn test_get_reminder_not_found() {
        let (persistence, _container) = setup().await;
        let result = persistence.get_reminder(Uuid::new_v4()).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_delete_reminder() {
        let (persistence, _container) = setup().await;
        let user_id = create_test_user(&persistence).await;
        let reminder = make_reminder(user_id);
        let reminder_id = reminder.id();

        persistence.save_reminder(reminder).await.unwrap();
        persistence.delete_reminder(reminder_id).await.unwrap();

        let found = persistence.get_reminder(reminder_id).await.unwrap();
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn test_update_reminder() {
        let (persistence, _container) = setup().await;
        let user_id = create_test_user(&persistence).await;
        let reminder = make_reminder(user_id);
        let reminder_id = reminder.id();

        persistence.save_reminder(reminder).await.unwrap();

        let mut fetched = persistence
            .get_reminder(reminder_id)
            .await
            .unwrap()
            .unwrap();
        fetched.update_title("Updated title".to_string());
        fetched.mark_sent();

        persistence.update_reminder(fetched).await.unwrap();

        let updated = persistence
            .get_reminder(reminder_id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(updated.title(), "Updated title");
        assert!(updated.is_sent());
    }

    #[tokio::test]
    async fn test_find_by_task_ids_empty() {
        let (persistence, _container) = setup().await;
        let result = persistence.find_by_task_ids(vec![]).await.unwrap();
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_find_by_task_ids_no_match() {
        let (persistence, _container) = setup().await;
        let result = persistence
            .find_by_task_ids(vec![Uuid::new_v4()])
            .await
            .unwrap();
        assert!(result.is_empty());
    }
}
