use crate::shared::persistence::schema;
use crate::shared::persistence::PostgresPersistence;
use crate::tasks::persistence::db_models::db_subtask::{DbSubtask, NewDbSubtask};
use crate::tasks::persistence::db_models::db_task::{DbTask, NewDbTask, UpdateDbTask};
use application::shared::traits::persistence_error::{PersistenceError, PersistenceResult};
use application::tasks::traits::task_persistence::TaskPersistence;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use domain::tasks::entities::task::Task;
use tracing::{debug, info, instrument};
use uuid::Uuid;

#[async_trait]
impl TaskPersistence for PostgresPersistence {
    #[instrument(skip(self))]
    async fn create_task(&self, task: Task) -> PersistenceResult<Uuid> {
        let subtasks: Vec<NewDbSubtask> = task
            .sub_tasks()
            .iter()
            .map(|s| NewDbSubtask::from_subtask(s, task.id(), task.user_id()))
            .collect();

        let task_id = self
            .with_transaction(move |conn| {
                let new_db_task = NewDbTask::from(task.clone());
                debug!("create_task: inserting priority={:?}", new_db_task.priority);
                let db_task = diesel::insert_into(schema::tasks::table)
                    .values(&new_db_task)
                    .returning(DbTask::as_returning())
                    .get_result(conn)?;

                debug!(
                    "create_task: returned db_task priority={:?}",
                    db_task.priority
                );

                if !subtasks.is_empty() {
                    diesel::insert_into(schema::subtasks::table)
                        .values(&subtasks)
                        .execute(conn)?;
                }

                Ok(db_task.id)
            })
            .await?;

        info!("Created task with id: {}", task_id);
        Ok(task_id)
    }

    #[instrument(skip(self))]
    async fn find_all(&self) -> PersistenceResult<Vec<Task>> {
        let tasks = self
            .with_transaction(move |conn| {
                let query = schema::tasks::table
                    .filter(schema::tasks::deleted_at.is_null())
                    .into_boxed();

                let db_tasks: Vec<DbTask> = query
                    .select(DbTask::as_select())
                    .order(schema::tasks::created_at.desc())
                    .load(conn)?;

                let mut tasks = Vec::with_capacity(db_tasks.len());
                for db_task in db_tasks {
                    debug!("Found task: {:?}", db_task);
                    debug!("Searching for subtasks");
                    let db_subtasks: Vec<DbSubtask> = schema::subtasks::table
                        .filter(schema::subtasks::task_id.eq(db_task.id))
                        .select(DbSubtask::as_select())
                        .order(schema::subtasks::sort_order.asc())
                        .load(conn)?;

                    let subtasks = db_subtasks.into_iter().map(|s| s.into()).collect();
                    tasks.push(db_task.into_task_with_subtasks(subtasks));
                }

                Ok(tasks)
            })
            .await?;

        Ok(tasks)
    }

    #[instrument(skip(self))]
    async fn find_by_id(&self, task_id: Uuid) -> PersistenceResult<Task> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let result = conn
            .interact(move |conn| {
                let db_task = schema::tasks::table
                    .filter(schema::tasks::id.eq(task_id))
                    .filter(schema::tasks::deleted_at.is_null())
                    .select(DbTask::as_select())
                    .first(conn)
                    .optional()?;

                match db_task {
                    None => Ok(None),
                    Some(db_task) => {
                        let db_subtasks: Vec<DbSubtask> = schema::subtasks::table
                            .filter(schema::subtasks::task_id.eq(db_task.id))
                            .select(DbSubtask::as_select())
                            .order(schema::subtasks::sort_order.asc())
                            .load(conn)?;
                        let subtasks = db_subtasks.into_iter().map(|s| s.into()).collect();
                        Ok(Some(db_task.into_task_with_subtasks(subtasks)))
                    }
                }
            })
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
            .map_err(|e: diesel::result::Error| PersistenceError::Unexpected(e.to_string()))?;

        match result {
            Some(task) => Ok(task),
            None => Err(PersistenceError::NotFound(format!(
                "Task with id {} not found",
                task_id
            ))),
        }
    }

    #[instrument(skip(self))]
    async fn find_scheduled_tasks(
        &self,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
        completed: Option<bool>,
    ) -> PersistenceResult<Vec<Task>> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let tasks = conn
            .interact(move |conn| {
                let mut query = schema::tasks::table
                    .filter(schema::tasks::deleted_at.is_null())
                    .filter(schema::tasks::scheduled_date.is_not_null())
                    .into_boxed();

                if let Some(from) = from {
                    query = query.filter(schema::tasks::scheduled_date.ge(from));
                }
                if let Some(to) = to {
                    query = query.filter(schema::tasks::scheduled_date.le(to));
                }

                match completed {
                    Some(true) => query = query.filter(schema::tasks::completed_at.is_not_null()),
                    Some(false) => query = query.filter(schema::tasks::completed_at.is_null()),
                    None => {}
                }

                let db_tasks: Vec<DbTask> = query
                    .select(DbTask::as_select())
                    .order(schema::tasks::scheduled_date.asc())
                    .load(conn)?;

                let mut tasks = Vec::with_capacity(db_tasks.len());
                for db_task in db_tasks {
                    let db_subtasks: Vec<DbSubtask> = schema::subtasks::table
                        .filter(schema::subtasks::task_id.eq(db_task.id))
                        .select(DbSubtask::as_select())
                        .order(schema::subtasks::sort_order.asc())
                        .load(conn)?;
                    let subtasks = db_subtasks.into_iter().map(|s| s.into()).collect();
                    tasks.push(db_task.into_task_with_subtasks(subtasks));
                }

                Ok(tasks)
            })
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
            .map_err(|e: diesel::result::Error| PersistenceError::Unexpected(e.to_string()))?;

        Ok(tasks)
    }

    #[instrument(skip(self))]
    async fn update_task(&self, task: Task) -> PersistenceResult<Task> {
        let task_id = task.id();
        let new_subtasks: Vec<NewDbSubtask> = task
            .sub_tasks()
            .iter()
            .map(|s| NewDbSubtask {
                id: s.id(),
                task_id,
                user_id: task.user_id(),
                title: s.title().to_string(),
                description: s.description().map(|d| d.to_string()),
                sort_order: s.sort_order(),
                completed_at: if s.is_completed() {
                    Some(Utc::now())
                } else {
                    None
                },
            })
            .collect();

        let update_dto = UpdateDbTask::from(task);
        debug!(
            "update_task id={} completed_at={:?} title={:?} priority={:?}",
            task_id, update_dto.completed_at, update_dto.title, update_dto.priority
        );

        let updated = self
            .with_transaction(move |conn| {
                let updated_db_task = diesel::update(schema::tasks::table)
                    .filter(schema::tasks::id.eq(task_id))
                    .filter(schema::tasks::deleted_at.is_null())
                    .set(&update_dto)
                    .returning(DbTask::as_returning())
                    .get_result(conn)
                    .optional()?;

                debug!(
                    "update_task result completed_at={:?}",
                    updated_db_task.as_ref().map(|t: &DbTask| t.completed_at)
                );

                let Some(updated_db_task) = updated_db_task else {
                    return Err(diesel::result::Error::NotFound);
                };

                debug!("Deleting subtasks for task_id: {}", task_id);
                diesel::delete(schema::subtasks::table)
                    .filter(schema::subtasks::task_id.eq(task_id))
                    .execute(conn)?;

                if !new_subtasks.is_empty() {
                    debug!("Inserting new subtasks for task_id: {}", task_id);
                    diesel::insert_into(schema::subtasks::table)
                        .values(&new_subtasks)
                        .execute(conn)?;
                }

                let db_subtasks: Vec<DbSubtask> = schema::subtasks::table
                    .filter(schema::subtasks::task_id.eq(task_id))
                    .select(DbSubtask::as_select())
                    .order(schema::subtasks::sort_order.asc())
                    .load(conn)?;

                let subtasks = db_subtasks.into_iter().map(|s| s.into()).collect();
                Ok(updated_db_task.into_task_with_subtasks(subtasks))
            })
            .await?;

        Ok(updated)
    }

    #[instrument(skip(self))]
    async fn delete_task(&self, task_id: Uuid) -> PersistenceResult<()> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let now = Utc::now();
        let affected_rows = conn
            .interact(move |conn| {
                diesel::update(schema::tasks::table)
                    .filter(schema::tasks::id.eq(task_id))
                    .filter(schema::tasks::deleted_at.is_null())
                    .set(schema::tasks::deleted_at.eq(now))
                    .execute(conn)
            })
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        match affected_rows > 0 {
            true => Ok(()),
            false => Err(PersistenceError::NotFound("Task not found".to_string())),
        }
    }
}
