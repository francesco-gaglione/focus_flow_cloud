use std::sync::Arc;

use domain::tasks::entities::subtask::Subtask;
use thiserror::Error;
use tracing::{error, info, instrument};
use uuid::Uuid;

use crate::repository_traits::{
    persistence_error::PersistenceError, task_persistence::TaskPersistence,
};

#[derive(Debug, Error, PartialEq)]
pub enum AddSubTaskError {
    #[error("Task not found: {0}")]
    TaskNotFound(Uuid),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type AddSubTaskResult<T> = Result<T, AddSubTaskError>;

#[derive(Debug)]
pub struct AddSubTaskCommand {
    pub task_id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
}

pub struct AddSubTaskUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl AddSubTaskUseCase {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, command: AddSubTaskCommand) -> AddSubTaskResult<Uuid> {
        info!("Finding task: {:?}", command.task_id);
        let mut task = self
            .task_persistence
            .find_by_id(command.task_id)
            .await
            .map_err(|e| match e {
                PersistenceError::NotFound(_) => AddSubTaskError::TaskNotFound(command.task_id),
                other => AddSubTaskError::PersistenceError(other),
            })?;

        if task.user_id() != command.user_id {
            error!(
                "Unauthorized attempt to add subtask to task: {:?} by user: {:?}",
                command.task_id, command.user_id
            );
            return Err(AddSubTaskError::Unauthorized);
        }

        let sort_order = task.sub_tasks().len() as i16;
        let subtask = Subtask::new(command.title, sort_order, command.description, None);
        let subtask_id = subtask.id();

        info!("Adding subtask to task: {:?}", command.task_id);
        task.add_subtask(subtask);

        self.task_persistence.update_task(task).await?;
        info!("Subtask added successfully, id: {:?}", subtask_id);

        Ok(subtask_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository_traits::persistence_error::PersistenceError;
    use crate::repository_traits::task_persistence::MockTaskPersistence;
    use domain::tasks::entities::task::Task;
    use domain::tasks::entities::task_schedule::TaskSchedule;
    use std::sync::Arc;

    fn make_task(user_id: Uuid) -> Task {
        Task::new(user_id, "Task".to_string(), TaskSchedule::Unscheduled, None)
    }

    #[tokio::test]
    async fn test_add_subtask_success() {
        let mut mock = MockTaskPersistence::new();
        let user_id = Uuid::new_v4();
        let task = make_task(user_id);
        let task_id = task.id();
        let task_clone = task.clone();

        mock.expect_find_by_id()
            .times(1)
            .returning(move |_| Ok(task_clone.clone()));

        mock.expect_update_task().times(1).returning(|t| Ok(t));

        let uc = AddSubTaskUseCase::new(Arc::new(mock));
        let result = uc
            .execute(AddSubTaskCommand {
                task_id,
                user_id,
                title: "Subtask".to_string(),
                description: None,
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_add_subtask_task_not_found() {
        let mut mock = MockTaskPersistence::new();
        let task_id = Uuid::new_v4();

        mock.expect_find_by_id()
            .times(1)
            .returning(move |_| Err(PersistenceError::NotFound("not found".to_string())));

        let uc = AddSubTaskUseCase::new(Arc::new(mock));
        let result = uc
            .execute(AddSubTaskCommand {
                task_id,
                user_id: Uuid::new_v4(),
                title: "Subtask".to_string(),
                description: None,
            })
            .await;

        assert!(matches!(
            result.unwrap_err(),
            AddSubTaskError::TaskNotFound(_)
        ));
    }

    #[tokio::test]
    async fn test_add_subtask_unauthorized() {
        let mut mock = MockTaskPersistence::new();
        let task_owner = Uuid::new_v4();
        let other_user = Uuid::new_v4();
        let task = make_task(task_owner);
        let task_id = task.id();

        mock.expect_find_by_id()
            .times(1)
            .returning(move |_| Ok(task.clone()));

        let uc = AddSubTaskUseCase::new(Arc::new(mock));
        let result = uc
            .execute(AddSubTaskCommand {
                task_id,
                user_id: other_user,
                title: "Subtask".to_string(),
                description: None,
            })
            .await;

        assert!(matches!(result.unwrap_err(), AddSubTaskError::Unauthorized));
    }

    #[tokio::test]
    async fn test_add_subtask_persistence_error_on_update() {
        let mut mock = MockTaskPersistence::new();
        let user_id = Uuid::new_v4();
        let task = make_task(user_id);
        let task_id = task.id();

        mock.expect_find_by_id()
            .times(1)
            .returning(move |_| Ok(task.clone()));

        mock.expect_update_task()
            .times(1)
            .returning(|_| Err(PersistenceError::Unexpected("db error".to_string())));

        let uc = AddSubTaskUseCase::new(Arc::new(mock));
        let result = uc
            .execute(AddSubTaskCommand {
                task_id,
                user_id,
                title: "Subtask".to_string(),
                description: None,
            })
            .await;

        assert!(matches!(
            result.unwrap_err(),
            AddSubTaskError::PersistenceError(_)
        ));
    }
}
