use crate::db_models::db_task::{NewDbTask, UpdateDbTask};
use crate::entities::task::Task;
use crate::repository::task_repository::TaskRepository;
use crate::services::service_error::ServiceError;
use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct CreateTaskCommand {
    pub name: String,
    pub description: Option<String>,
    pub category_id: Option<String>,
    pub scheduled_date: Option<NaiveDate>,
}

#[derive(Clone, Debug)]
pub struct UpdateTaskCommand {
    pub task_id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
    pub category_id: Option<String>,
    pub scheduled_date: Option<NaiveDate>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug)]
pub struct TaskService {
    task_repository: TaskRepository,
}

impl TaskService {
    pub fn new(task_repository: TaskRepository) -> Self {
        Self { task_repository }
    }

    pub async fn get_all_tasks(&self) -> Result<Vec<Task>, ServiceError> {
        let tasks = self.task_repository.find_all().await.map_err(|e| {
            log::error!("{:?}", e);
            ServiceError::RepositoryError(e)
        })?;

        Ok(tasks
            .iter()
            .map(|t| Task {
                id: t.id,
                category_id: t.category_id,
                name: t.name.clone(),
                description: t.description.clone(),
                scheduled_date: t.scheduled_date.clone(),
                created_at: t.created_at.clone(),
                completed_at: t.completed_at.clone(),
            })
            .collect())
    }

    pub async fn create_task(&self, command: CreateTaskCommand) -> Result<Task, ServiceError> {
        let category_id = match command.category_id {
            None => None,
            Some(category_id) => {
                let id = Uuid::parse_str(category_id.as_str())
                    .map_err(|e| ServiceError::GenericError)?;
                Some(id)
            }
        };
        let db_task = self
            .task_repository
            .create(NewDbTask {
                category_id,
                name: command.name,
                description: command.description,
                scheduled_date: command.scheduled_date,
            })
            .await
            .map_err(|e| ServiceError::RepositoryError(e))?;

        Ok(db_task.into())
    }

    pub async fn delete_tasks(&self, task_ids: Vec<Uuid>) -> Result<Vec<Uuid>, ServiceError> {
        let mut deleted_ids: Vec<Uuid> = vec![];
        for task_id in task_ids {
            let res = self.task_repository.hard_delete(task_id).await;
            log::debug!("{:?}", res);
            match res {
                Ok(_) => {
                    deleted_ids.push(task_id);
                }
                Err(err) => {
                    log::error!("{:?}", err);
                }
            }
        }
        Ok(deleted_ids)
    }

    pub async fn update_task(&self, command: UpdateTaskCommand) -> Result<Task, ServiceError> {
        let db_task = self
            .task_repository
            .update(
                command.task_id,
                UpdateDbTask {
                    category_id: command
                        .category_id
                        .map(|id| Uuid::parse_str(id.as_str()).unwrap()), // should be safe due to validation
                    name: command.name,
                    description: command.description,
                    scheduled_date: command.scheduled_date,
                    completed_at: command.completed_at,
                },
            )
            .await
            .map_err(|e| ServiceError::RepositoryError(e))?;

        match db_task {
            None => Err(ServiceError::GenericError),
            Some(db_task) => Ok(db_task.into()),
        }
    }
}
