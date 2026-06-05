use std::sync::Arc;

use thiserror::Error;
use tracing::{error, info, instrument};
use uuid::Uuid;

use crate::shared::traits::persistence_error::PersistenceError;
use crate::tasks::traits::task_persistence::TaskPersistence;

#[derive(Debug, Error, PartialEq)]
pub enum UpdateSubTaskError {
    #[error("Task not found: {0}")]
    TaskNotFound(Uuid),

    #[error("SubTask not found: {0}")]
    SubTaskNotFound(Uuid),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Sub-tasks must be completed before marking task as completed")]
    UncompletedSubTasks,

    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type UpdateSubTaskResult<T> = Result<T, UpdateSubTaskError>;

#[derive(Debug)]
pub struct UpdateSubTaskCommand {
    pub task_id: Uuid,
    pub sub_task_id: Uuid,
    pub user_id: Uuid,
    pub completed: Option<bool>,
}

pub struct UpdateSubTaskUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
}

impl UpdateSubTaskUseCase {
    pub fn new(task_persistence: Arc<dyn TaskPersistence>) -> Self {
        Self { task_persistence }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, command: UpdateSubTaskCommand) -> UpdateSubTaskResult<()> {
        info!("Finding task: {:?}", command.task_id);
        let mut task = self.task_persistence.find_by_id(command.task_id).await?;

        if task.user_id() != command.user_id {
            error!(
                "Unauthorized attempt to complete subtask: {:?} by user: {:?}",
                command.sub_task_id, command.user_id
            );
            return Err(UpdateSubTaskError::Unauthorized);
        }

        if let Some(completed) = command.completed {
            let sub_task = task
                .sub_tasks_mut()
                .iter_mut()
                .find(|s| s.id() == command.sub_task_id)
                .ok_or_else(|| {
                    error!("Subtask not found: {:?}", command.sub_task_id);
                    UpdateSubTaskError::SubTaskNotFound(command.sub_task_id)
                })?;
            if completed {
                info!("Completing subtask: {:?}", command.sub_task_id);
                sub_task.mark_completed();
            } else {
                sub_task.mark_incomplete();
            }
        }

        info!("Updating subtask: {:?}", command.task_id);
        self.task_persistence.update_task(task).await?;
        info!("Subtask updated successfully: {:?}", command.task_id);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::traits::persistence_error::PersistenceError;
    use crate::tasks::traits::task_persistence::MockTaskPersistence;
    use domain::tasks::entities::subtask::Subtask;
    use domain::tasks::entities::task::Task;
    use domain::tasks::entities::task_schedule::TaskSchedule;
    use std::sync::Arc;

    fn make_task_with_subtask(user_id: Uuid) -> (Task, Uuid) {
        let mut task = Task::new(user_id, "Task".to_string(), TaskSchedule::Unscheduled, None);
        let subtask = Subtask::new("Sub".to_string(), 0, None, None);
        let subtask_id = subtask.id();
        task.add_subtask(subtask);
        (task, subtask_id)
    }

    #[tokio::test]
    async fn test_complete_subtask_success() {
        let mut mock = MockTaskPersistence::new();
        let user_id = Uuid::new_v4();
        let (task, subtask_id) = make_task_with_subtask(user_id);
        let task_id = task.id();

        mock.expect_find_by_id()
            .times(1)
            .returning(move |_| Ok(task.clone()));

        mock.expect_update_task().times(1).returning(|t| Ok(t));

        let uc = UpdateSubTaskUseCase::new(Arc::new(mock));
        let result = uc
            .execute(UpdateSubTaskCommand {
                task_id,
                sub_task_id: subtask_id,
                user_id,
                completed: Some(true),
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_subtask_no_completed_field() {
        let mut mock = MockTaskPersistence::new();
        let user_id = Uuid::new_v4();
        let (task, subtask_id) = make_task_with_subtask(user_id);
        let task_id = task.id();

        mock.expect_find_by_id()
            .times(1)
            .returning(move |_| Ok(task.clone()));

        mock.expect_update_task().times(1).returning(|t| Ok(t));

        let uc = UpdateSubTaskUseCase::new(Arc::new(mock));
        let result = uc
            .execute(UpdateSubTaskCommand {
                task_id,
                sub_task_id: subtask_id,
                user_id,
                completed: None,
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_subtask_unauthorized() {
        let mut mock = MockTaskPersistence::new();
        let owner = Uuid::new_v4();
        let other = Uuid::new_v4();
        let (task, subtask_id) = make_task_with_subtask(owner);
        let task_id = task.id();

        mock.expect_find_by_id()
            .times(1)
            .returning(move |_| Ok(task.clone()));

        let uc = UpdateSubTaskUseCase::new(Arc::new(mock));
        let result = uc
            .execute(UpdateSubTaskCommand {
                task_id,
                sub_task_id: subtask_id,
                user_id: other,
                completed: Some(true),
            })
            .await;

        assert!(matches!(
            result.unwrap_err(),
            UpdateSubTaskError::Unauthorized
        ));
    }

    #[tokio::test]
    async fn test_update_subtask_not_found() {
        let mut mock = MockTaskPersistence::new();
        let user_id = Uuid::new_v4();
        let (task, _) = make_task_with_subtask(user_id);
        let task_id = task.id();
        let wrong_subtask_id = Uuid::new_v4();

        mock.expect_find_by_id()
            .times(1)
            .returning(move |_| Ok(task.clone()));

        let uc = UpdateSubTaskUseCase::new(Arc::new(mock));
        let result = uc
            .execute(UpdateSubTaskCommand {
                task_id,
                sub_task_id: wrong_subtask_id,
                user_id,
                completed: Some(true),
            })
            .await;

        assert!(matches!(
            result.unwrap_err(),
            UpdateSubTaskError::SubTaskNotFound(_)
        ));
    }

    #[tokio::test]
    async fn test_update_subtask_task_not_found() {
        let mut mock = MockTaskPersistence::new();

        mock.expect_find_by_id()
            .times(1)
            .returning(|_| Err(PersistenceError::NotFound("not found".to_string())));

        let uc = UpdateSubTaskUseCase::new(Arc::new(mock));
        let result = uc
            .execute(UpdateSubTaskCommand {
                task_id: Uuid::new_v4(),
                sub_task_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                completed: Some(true),
            })
            .await;

        assert!(matches!(
            result.unwrap_err(),
            UpdateSubTaskError::PersistenceError(_)
        ));
    }
}
