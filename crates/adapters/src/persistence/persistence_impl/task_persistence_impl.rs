use crate::persistence::db_models::db_task::{DbTask, NewDbTask, UpdateDbTask};
use crate::persistence::PostgresPersistence;
use crate::persistence::schema;
use async_trait::async_trait;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use domain::entities::task::Task;
use domain::error::persistence_error::{PersistenceError, PersistenceResult};
use domain::traits::task_persistence::TaskPersistence;
use tracing::info;
use uuid::Uuid;

#[async_trait]
impl TaskPersistence for PostgresPersistence {
    async fn create_task(&self, task: Task) -> PersistenceResult<Uuid> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let result = conn
            .interact(move |conn| {
                diesel::insert_into(schema::tasks::table)
                    .values(&NewDbTask::from(task))
                    .returning(DbTask::as_returning())
                    .get_result(conn)
            })
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        info!("Created task with id: {}", result.id);
        Ok(result.id)
    }

    async fn find_all(&self) -> PersistenceResult<Vec<Task>> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let result = conn
            .interact(move |conn| {
                schema::tasks::table
                    .filter(schema::tasks::deleted_at.is_null())
                    .select(DbTask::as_select())
                    .order(schema::tasks::created_at.desc())
                    .load(conn)
            })
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let tasks: Vec<Task> = result.into_iter().map(|c| c.into()).collect();
        Ok(tasks)
    }

    async fn find_orphan_tasks(&self) -> PersistenceResult<Vec<Task>> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let result = conn
            .interact(move |conn| {
                schema::tasks::table
                    .filter(schema::tasks::category_id.is_null())
                    .select(DbTask::as_select())
                    .load(conn)
            })
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let tasks: Vec<Task> = result.into_iter().map(|c| c.into()).collect();
        Ok(tasks)
    }

    async fn find_by_category_id(&self, category_id: Uuid) -> PersistenceResult<Vec<Task>> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let result = conn
            .interact(move |conn| {
                schema::tasks::table
                    .filter(schema::tasks::category_id.eq(category_id))
                    .select(DbTask::as_select())
                    .load(conn)
            })
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let tasks = result.into_iter().map(|c| c.into()).collect();

        Ok(tasks)
    }

    async fn find_by_id(&self, task_id: Uuid) -> PersistenceResult<Task> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let result = conn
            .interact(move |conn| {
                schema::tasks::table
                    .filter(schema::tasks::id.eq(task_id))
                    .filter(schema::tasks::deleted_at.is_null())
                    .select(DbTask::as_select())
                    .first(conn)
                    .optional()
            })
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        match result {
            Some(db_task) => Ok(db_task.into()),
            None => Err(PersistenceError::NotFound(format!(
                "Task with id {} not found",
                task_id
            ))),
        }
    }

    async fn update_task(&self, task: Task) -> PersistenceResult<Task> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let task_id = task.id(); // Ensure we use ID from entity

        let result = conn
            .interact(move |conn| {
                diesel::update(schema::tasks::table)
                    .filter(schema::tasks::id.eq(task_id))
                    .filter(schema::tasks::deleted_at.is_null())
                    .set(&UpdateDbTask::from(task))
                    .returning(DbTask::as_returning())
                    .get_result(conn)
                    .optional()
            })
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        match result {
            None => Err(PersistenceError::Unexpected("Task not updated".to_string())),
            Some(updated) => Ok(updated.into()),
        }
    }

    async fn delete_task(&self, task_id: Uuid) -> PersistenceResult<()> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let affected_rows = conn
            .interact(move |conn| {
                diesel::delete(schema::tasks::table)
                    .filter(schema::tasks::id.eq(task_id))
                    .execute(conn)
            })
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        match affected_rows > 0 {
            true => Ok(()),
            false => Err(PersistenceError::NotFound("Task not found".to_string())), // Changed to NotFound or Unexpected
        }
    }
}
