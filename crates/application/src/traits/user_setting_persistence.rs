use crate::app_error::AppResult;
use async_trait::async_trait;
use domain::entities::user_setting::UserSetting;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UserSettingPersistence: Send + Sync {
    async fn find_all(&self) -> AppResult<Vec<UserSetting>>;
    async fn update_setting(&self, key: String, value: String) -> AppResult<()>;
    async fn create_setting(&self, key: String, value: String) -> AppResult<()>;
}

#[cfg(test)]
mod tests {
    use crate::app_error::AppError;

    use super::*;

    #[tokio::test]
    async fn test_find_all() {
        let mut mock = MockUserSettingPersistence::new();
        mock.expect_find_all().returning(|| Ok(vec![]));
        let result = mock.find_all().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_setting() {
        let mut mock = MockUserSettingPersistence::new();
        mock.expect_update_setting().returning(|_, _| Ok(()));
        let result = mock
            .update_setting("key".to_string(), "value".to_string())
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_setting() {
        let mut mock = MockUserSettingPersistence::new();
        mock.expect_create_setting().returning(|_, _| Ok(()));
        let result = mock
            .create_setting("key".to_string(), "value".to_string())
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_setting_error() {
        let mut mock = MockUserSettingPersistence::new();
        mock.expect_create_setting()
            .returning(|_, _| Err(AppError::GenericError("test".to_string())));
        let result = mock
            .create_setting("key".to_string(), "value".to_string())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_setting_error() {
        let mut mock = MockUserSettingPersistence::new();
        mock.expect_update_setting()
            .returning(|_, _| Err(AppError::GenericError("test".to_string())));
        let result = mock
            .update_setting("key".to_string(), "value".to_string())
            .await;
        assert!(result.is_err())
    }

    #[tokio::test]
    async fn test_find_all_error() {
        let mut mock = MockUserSettingPersistence::new();
        mock.expect_find_all()
            .returning(|| Err(AppError::GenericError("test".to_string())));
        let result = mock.find_all().await;
        assert!(result.is_err());
    }
}
