use crate::app_error::AppResult;
use crate::use_cases::persistance_command::create_task_data::CreateTaskData;
use crate::use_cases::persistance_command::update_task_data::UpdateTaskData;
use async_trait::async_trait;
use domain::entities::task::Task;
use uuid::Uuid;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait TaskPersistence: Send + Sync {
    async fn create_task(&self, task: CreateTaskData) -> AppResult<Uuid>;
    async fn find_all(&self) -> AppResult<Vec<Task>>;
    async fn find_orphan_tasks(&self) -> AppResult<Vec<Task>>;
    async fn find_by_category_id(&self, category_id: Uuid) -> AppResult<Vec<Task>>;
    async fn find_by_id(&self, task_id: Uuid) -> AppResult<Task>;
    async fn update_task(&self, task_id: Uuid, task: UpdateTaskData) -> AppResult<Task>;
    async fn delete_task(&self, task: Uuid) -> AppResult<()>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_task() {
        let mut mock = MockTaskPersistence::new();
        mock.expect_create_task()
            .times(1)
            .returning(|_| Ok(Uuid::new_v4()));
        let task = CreateTaskData::new("Test Task".to_string(), None, None, None);
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
    async fn test_find_orphan_tasks() {
        let mut mock = MockTaskPersistence::new();
        mock.expect_find_orphan_tasks()
            .times(1)
            .returning(|| Ok(vec![]));
        let result = mock.find_orphan_tasks().await;
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
            Ok(Task::new(
                Uuid::new_v4(),
                None,
                "name".to_string(),
                None,
                None,
                None,
            ))
        });
        let result = mock.find_by_id(Uuid::new_v4()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_task() {
        let mut mock = MockTaskPersistence::new();
        mock.expect_update_task().times(1).returning(|_, _| {
            Ok(Task::new(
                Uuid::new_v4(),
                None,
                "name".to_string(),
                None,
                None,
                None,
            ))
        });
        let task = UpdateTaskData::new(Some(Uuid::new_v4()), None, None, None, None);
        let result = mock.update_task(Uuid::new_v4(), task).await;
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
