use crate::entities::user::User;
use crate::error::persistence_error::PersistenceResult;
use async_trait::async_trait;
use uuid::Uuid;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UserPersistence: Send + Sync {
    async fn create_user(&self, user: User) -> PersistenceResult<Uuid>;

    async fn find_user_by_id(&self, user_id: Uuid) -> PersistenceResult<User>;

    async fn find_user_by_username(&self, username: &str) -> PersistenceResult<User>;

    async fn update_user(&self, user: User) -> PersistenceResult<()>;

    async fn delete_user(&self, user_id: Uuid) -> PersistenceResult<()>;

    async fn is_user_admin(&self, user_id: Uuid) -> PersistenceResult<bool>;
}
