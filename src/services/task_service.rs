use crate::entities::task::Task;
use crate::repository::task_repository::TaskRepository;
use crate::services::service_error::ServiceError;

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
}
