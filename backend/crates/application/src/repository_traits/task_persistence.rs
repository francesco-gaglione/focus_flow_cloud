use crate::repository_traits::persistence_error::PersistenceResult;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::tasks::entities::task::Task;
use uuid::Uuid;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait TaskPersistence: Send + Sync {
    async fn create_task(&self, task: Task) -> PersistenceResult<Uuid>;

    async fn find_all(&self) -> PersistenceResult<Vec<Task>>;

    async fn find_by_id(&self, task_id: Uuid) -> PersistenceResult<Task>;

    /// Finds tasks that have a scheduled date.
    ///
    ///# Arguments
    ///
    ///* `from` - If provided, only returns tasks scheduled at or after this time.
    ///* `to` - If provided, only returns tasks scheduled at or before this time.
    ///* `completed` - Filters by completion status. `None` returns all tasks,
    ///  `Some(true)` only completed ones, `Some(false)` only pending ones.
    async fn find_scheduled_tasks(
        &self,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
        completed: Option<bool>,
    ) -> PersistenceResult<Vec<Task>>;

    async fn update_task(&self, task: Task) -> PersistenceResult<Task>;

    async fn delete_task(&self, task_id: Uuid) -> PersistenceResult<()>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::tasks::entities::task_schedule::TaskSchedule;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_task() {
        let mut mock = MockTaskPersistence::new();
        mock.expect_create_task()
            .times(1)
            .returning(|_| Ok(Uuid::new_v4()));
        let task = Task::new(
            Uuid::new_v4(),
            "Title".to_string(),
            TaskSchedule::Unscheduled,
            None,
        );
        let result = mock.create_task(task).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_find_all() {
        let mut mock = MockTaskPersistence::new();
        mock.expect_find_all().times(1).returning(|| Ok(vec![]));
        let result = mock.find_all().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_find_by_id() {
        let mut mock = MockTaskPersistence::new();
        mock.expect_find_by_id().times(1).returning(|_| {
            Ok(Task::new(
                Uuid::new_v4(),
                "title".to_string(),
                TaskSchedule::Unscheduled,
                None,
            ))
        });
        let result = mock.find_by_id(Uuid::new_v4()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_find_scheduled_tasks() {
        let mut mock = MockTaskPersistence::new();
        mock.expect_find_scheduled_tasks()
            .times(1)
            .returning(|_, _, _| {
                Ok(vec![Task::new(
                    Uuid::new_v4(),
                    "title".to_string(),
                    TaskSchedule::Unscheduled,
                    None,
                )])
            });
        let result = mock.find_scheduled_tasks(None, None, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_task() {
        let mut mock = MockTaskPersistence::new();
        mock.expect_update_task().times(1).returning(|_| {
            Ok(Task::new(
                Uuid::new_v4(),
                "title".to_string(),
                TaskSchedule::Unscheduled,
                None,
            ))
        });
        let task = Task::new(
            Uuid::new_v4(),
            "title".to_string(),
            TaskSchedule::Unscheduled,
            None,
        );
        let result = mock.update_task(task).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_task() {
        let mut mock = MockTaskPersistence::new();
        mock.expect_delete_task().times(1).returning(|_| Ok(()));
        let result = mock.delete_task(Uuid::new_v4()).await;
        assert!(result.is_ok());
    }
}
