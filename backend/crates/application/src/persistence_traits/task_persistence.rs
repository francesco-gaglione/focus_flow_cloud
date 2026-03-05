use crate::persistence_traits::persistence_error::PersistenceResult;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::entities::task::Task;
use uuid::Uuid;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait TaskPersistence: Send + Sync {
    async fn create_task(&self, task: Task) -> PersistenceResult<Uuid>;

    async fn find_all(&self, completed: bool) -> PersistenceResult<Vec<Task>>;

    async fn find_orphan_tasks(&self, completed: bool) -> PersistenceResult<Vec<Task>>;

    async fn find_by_category_id(&self, category_id: Uuid) -> PersistenceResult<Vec<Task>>;

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
    use chrono::Duration;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_task() {
        let mut mock = MockTaskPersistence::new();
        mock.expect_create_task()
            .times(1)
            .returning(|_| Ok(Uuid::new_v4()));
        let task = Task::create(
            Uuid::new_v4(),
            None,
            "Test Task".to_string(),
            None,
            None,
            None,
        )
        .unwrap();
        let result = mock.create_task(task).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_find_all() {
        let mut mock = MockTaskPersistence::new();
        mock.expect_find_all()
            .with(mockall::predicate::eq(false))
            .times(1)
            .returning(|_| Ok(vec![]));
        let result = mock.find_all(false).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_find_orphan_tasks() {
        let mut mock = MockTaskPersistence::new();
        mock.expect_find_orphan_tasks()
            .with(mockall::predicate::eq(true))
            .times(1)
            .returning(|_| Ok(vec![]));
        let result = mock.find_orphan_tasks(true).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_find_by_category_id() {
        let mut mock = MockTaskPersistence::new();
        mock.expect_find_by_category_id()
            .times(1)
            .returning(|_| Ok(vec![]));
        let result = mock.find_by_category_id(Uuid::new_v4()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_find_by_id() {
        let mut mock = MockTaskPersistence::new();
        mock.expect_find_by_id().times(1).returning(|_| {
            Ok(Task::reconstitute(
                Uuid::new_v4(),
                Uuid::new_v4(),
                None,
                "name".to_string(),
                None,
                None,
                None,
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
                Ok(vec![Task::reconstitute(
                    Uuid::new_v4(),
                    Uuid::new_v4(),
                    None,
                    "name".to_string(),
                    None,
                    Some(Utc::now()),
                    Some(Utc::now() + Duration::minutes(5)),
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
            Ok(Task::reconstitute(
                Uuid::new_v4(),
                Uuid::new_v4(),
                None,
                "name".to_string(),
                None,
                None,
                None,
                None,
            ))
        });
        let task = Task::create(
            Uuid::new_v4(),
            None,
            "Updated".to_string(),
            None,
            None,
            None,
        )
        .unwrap();
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
