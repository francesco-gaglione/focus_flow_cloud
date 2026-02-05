#[cfg(test)]
use domain::entities::{
    category::Category,
    focus_session::{FocusSession, SessionFilter},
    task::Task,
    user::User,
    user_setting::UserSetting,
};
#[cfg(test)]
use domain::error::persistence_error::PersistenceResult;
#[cfg(test)]
use domain::traits::password_hasher::{HashingResult, PasswordHasher};
#[cfg(test)]
use domain::traits::password_policy::{PasswordPolicy, PasswordPolicyResult};
#[cfg(test)]
use domain::traits::{
    category_persistence::CategoryPersistence, focus_session_persistence::FocusSessionPersistence,
    task_persistence::TaskPersistence, user_persistence::UserPersistence,
    user_setting_persistence::UserSettingPersistence,
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
         async fn update_setting(&self, user_id: Uuid, key: String, value: String) -> PersistenceResult<()>;
         async fn create_setting(&self, user_id: Uuid, key: String, value: String) -> PersistenceResult<()>;
    }
}

#[cfg(test)]
mock! {
    pub UserPersistence {}
    #[async_trait::async_trait]
    impl UserPersistence for UserPersistence {
        async fn find_user_by_id(&self, user_id: Uuid) -> PersistenceResult<User>;
        async fn find_user_by_username(&self, username: &str) -> PersistenceResult<User>;
        async fn create_user(&self, user: User) -> PersistenceResult<Uuid>;
        async fn update_user(&self, user: User) -> PersistenceResult<()>;
        async fn delete_user(&self, user_id: Uuid) -> PersistenceResult<()>;
        async fn is_user_admin(&self, user_id: Uuid) -> PersistenceResult<bool>;
    }
}

#[cfg(test)]
mock! {
    pub PasswordPolicy {}
    impl PasswordPolicy for PasswordPolicy {
        fn validate(&self, password: &str) -> PasswordPolicyResult<()>;
    }
}

#[cfg(test)]
mock! {
    pub PasswordHasher {}
    impl PasswordHasher for PasswordHasher {
        fn hash_password(&self, password: &str) -> HashingResult<String>;
        fn verify_password(&self, password: &str, hashed_password: &str) -> HashingResult<bool>;
    }
}
