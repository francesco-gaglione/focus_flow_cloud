#[cfg(test)]
use domain::entities::{
    category::Category,
    focus_session::{FocusSession, SessionFilter},
    task::Task,
    user_setting::UserSetting,
};
#[cfg(test)]
use domain::error::persistence_error::PersistenceResult;
#[cfg(test)]
use domain::traits::{
    category_persistence::CategoryPersistence, focus_session_persistence::FocusSessionPersistence,
    task_persistence::TaskPersistence, user_setting_persistence::UserSettingPersistence,
};
#[cfg(test)]
use mockall::mock;
#[cfg(test)]
use uuid::Uuid;

#[cfg(test)]
mock! {
    pub CategoryPersistence {}
    #[async_trait::async_trait]
    impl CategoryPersistence for CategoryPersistence {
        async fn create_category(&self, category: Category) -> PersistenceResult<Uuid>;
        async fn find_all(&self) -> PersistenceResult<Vec<Category>>;
        async fn find_by_id(&self, category_id: Uuid) -> PersistenceResult<Category>;
        async fn update_category(&self, category: Category) -> PersistenceResult<Category>;
        async fn delete_category_by_id(&self, category_id: Uuid) -> PersistenceResult<()>;
    }
}

#[cfg(test)]
mock! {
    pub TaskPersistence {}
    #[async_trait::async_trait]
    impl TaskPersistence for TaskPersistence {
        async fn create_task(&self, task: Task) -> PersistenceResult<Uuid>;
        async fn find_all(&self) -> PersistenceResult<Vec<Task>>;
        async fn find_orphan_tasks(&self) -> PersistenceResult<Vec<Task>>;
        async fn find_by_category_id(&self, category_id: Uuid) -> PersistenceResult<Vec<Task>>;
        async fn find_by_id(&self, task_id: Uuid) -> PersistenceResult<Task>;
        async fn update_task(&self, task: Task) -> PersistenceResult<Task>;
        async fn delete_task(&self, task_id: Uuid) -> PersistenceResult<()>;
    }
}

#[cfg(test)]
mock! {
    pub FocusSessionPersistence {}
    #[async_trait::async_trait]
    impl FocusSessionPersistence for FocusSessionPersistence {
        async fn find_by_filters(&self, filters: SessionFilter) -> PersistenceResult<Vec<FocusSession>>;
        async fn create_manual_session(&self, session: FocusSession) -> PersistenceResult<FocusSession>;
        async fn create_session(&self, session: FocusSession) -> PersistenceResult<FocusSession>;
        async fn update_session(&self, session: FocusSession) -> PersistenceResult<()>;
        async fn find_session_by_id(&self, session_id: Uuid) -> PersistenceResult<FocusSession>;
    }
}

#[cfg(test)]
mock! {
    pub UserSettingPersistence {}
    #[async_trait::async_trait]
    impl UserSettingPersistence for UserSettingPersistence {
         async fn find_all(&self) -> PersistenceResult<Vec<UserSetting>>;
         async fn update_setting(&self, key: String, value: String) -> PersistenceResult<()>;
         async fn create_setting(&self, key: String, value: String) -> PersistenceResult<()>;
    }
}
