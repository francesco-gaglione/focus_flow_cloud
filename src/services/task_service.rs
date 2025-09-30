use crate::db_models::db_task::NewDbTask;
use crate::entities::task::Task;
use crate::repository::task_repository::TaskRepository;
use crate::services::service_error::ServiceError;
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct CreateTaskCommand {
    pub name: String,
    pub description: Option<String>,
    pub category_id: Option<String>,
    pub scheduled_date: Option<NaiveDate>,
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
}
