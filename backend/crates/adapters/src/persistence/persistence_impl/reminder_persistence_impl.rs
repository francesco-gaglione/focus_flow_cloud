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
use uuid::Uuid;

#[async_trait]
impl ReminderPersistence for PostgresPersistence {
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
}
