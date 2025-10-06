use crate::adapters::persistence::PostgresPersistence;
use crate::adapters::persistence::db_models::db_task::{DbTask, NewDbTask, UpdateDbTask};
use crate::application::app_error::{AppError, AppResult};
use crate::application::traits::TaskPersistence;
use crate::application::use_cases::commands::create_task::CreateTaskCommand;
use crate::application::use_cases::commands::update_task::UpdateTaskCommand;
use crate::domain::entities::task::Task;
use async_trait::async_trait;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use tracing::{debug, info};
use uuid::Uuid;
use crate::adapters::schema;

#[async_trait]
impl TaskPersistence for PostgresPersistence {
    async fn create_task(&self, create_task_command: &CreateTaskCommand) -> AppResult<Uuid> {
        let task = create_task_command.clone();
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let result = conn
            .interact(move |conn| {
                diesel::insert_into(schema::tasks::table)
                    .values(&NewDbTask::from(task))
                    .returning(DbTask::as_returning())
                    .get_result(conn)
            })
            .await
            .map_err(|e| AppError::Database(e.to_string()))??;

        info!("Created task with id: {}", result.id);
        Ok(result.id)
    }

    async fn find_all(&self) -> AppResult<Vec<Task>> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let result = conn
            .interact(move |conn| {
                schema::tasks::table
                    .filter(schema::tasks::deleted_at.is_null())
                    .select(DbTask::as_select())
                    .order(schema::tasks::created_at.desc())
                    .load(conn)
            })
            .await
            .map_err(|e| AppError::Database(e.to_string()))??;

        let tasks: Vec<Task> = result.into_iter().map(|c| c.into()).collect();
        Ok(tasks)
    }

    async fn find_orphan_tasks(&self) -> AppResult<Vec<Task>> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let result = conn
            .interact(move |conn| {
                schema::tasks::table
                    .filter(schema::tasks::category_id.is_null())
                    .select(DbTask::as_select())
                    .load(conn)
            })
            .await
            .map_err(|e| AppError::Database(e.to_string()))??;

        let tasks: Vec<Task> = result.into_iter().map(|c| c.into()).collect();
        Ok(tasks)
    }

    async fn find_by_category_id(&self, category_id: Uuid) -> AppResult<Vec<Task>> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let result = conn
            .interact(move |conn| {
                schema::tasks::table
                    .filter(schema::tasks::category_id.eq(category_id))
                    .select(DbTask::as_select())
                    .load(conn)
            })
            .await
            .map_err(|e| AppError::Database(e.to_string()))??;

        let tasks = result.into_iter().map(|c| c.into()).collect();

        Ok(tasks)
    }

    async fn update_task(&self, task: &UpdateTaskCommand) -> AppResult<Task> {
        let task = task.clone();
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let result = conn
            .interact(move |conn| {
                diesel::update(schema::tasks::table)
                    .filter(schema::tasks::id.eq(task.id))
                    .filter(schema::tasks::deleted_at.is_null())
                    .set(&UpdateDbTask::from(task))
                    .returning(DbTask::as_returning())
                    .get_result(conn)
                    .optional()
            })
            .await
            .map_err(|e| AppError::Database(e.to_string()))??;

        match result {
            None => Err(AppError::GenericError("Task not updated".to_string())),
            Some(updated) => Ok(updated.into()),
        }
    }

    async fn delete_task(&self, task_id: Uuid) -> AppResult<()> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let affected_rows = conn
            .interact(move |conn| {
                diesel::delete(schema::tasks::table)
                    .filter(schema::tasks::id.eq(task_id))
                    .execute(conn)
            })
            .await
            .map_err(|e| AppError::Database(e.to_string()))??;

        match affected_rows > 0 {
            true => Ok(()),
            false => Err(AppError::Database("Task not deleted".to_string())),
        }
    }
}
