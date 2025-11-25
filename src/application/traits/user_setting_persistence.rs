use crate::application::app_error::AppResult;
use crate::domain::entities::user_setting::UserSetting;
use async_trait::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UserSettingPersistence: Send + Sync {
    async fn find_all(&self) -> AppResult<Vec<UserSetting>>;
    async fn update_setting(&self, key: String, value: String) -> AppResult<()>;
    async fn create_setting(&self, key: String, value: String) -> AppResult<()>;
}
