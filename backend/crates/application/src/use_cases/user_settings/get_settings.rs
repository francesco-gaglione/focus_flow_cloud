use crate::persistence_traits::persistence_error::PersistenceError;
use crate::persistence_traits::user_setting_persistence::UserSettingPersistence;
use domain::entities::user_setting::UserSetting;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum GetSettingsError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type GetSettingsResult<T> = Result<T, GetSettingsError>;

pub struct GetSettingsUseCase {
    setting_persistence: Arc<dyn UserSettingPersistence>,
}

#[derive(Debug, Clone)]
pub struct UserSettingOutput {
    pub key: String,
    pub value: Option<String>,
}

impl From<&UserSetting> for UserSettingOutput {
    fn from(value: &UserSetting) -> Self {
        Self {
            key: value.key(),
            value: value.value(),
        }
    }
}

impl GetSettingsUseCase {
    pub fn new(setting_persistence: Arc<dyn UserSettingPersistence>) -> Self {
        Self {
            setting_persistence,
        }
    }

    pub async fn execute(&self) -> GetSettingsResult<Vec<UserSettingOutput>> {
        Ok(self
            .setting_persistence
            .find_all()
            .await?
            .iter()
            .map(|s| s.into())
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence_traits::user_setting_persistence::MockUserSettingPersistence;
    use domain::entities::user_setting::UserSetting;

    #[tokio::test]
    async fn test_get_settings_success() {
        let mut mock_persistence = MockUserSettingPersistence::new();
        let expected_settings = vec![UserSetting::new(
            "theme".to_string(),
            Some("dark".to_string()),
        )];
        let returned_settings = expected_settings.clone();

        mock_persistence
            .expect_find_all()
            .returning(move || Ok(returned_settings.clone()));

        let use_case = GetSettingsUseCase::new(Arc::new(mock_persistence));
        let result = use_case.execute().await;

        assert!(result.is_ok());
        let result = result.unwrap().get(0).unwrap().clone();
        assert_eq!(result.key, expected_settings.get(0).unwrap().key());
        assert_eq!(result.value, expected_settings.get(0).unwrap().value());
    }

    #[tokio::test]
    async fn test_get_settings_persistence_error() {
        let mut mock_persistence = MockUserSettingPersistence::new();
        mock_persistence
            .expect_find_all()
            .returning(|| Err(PersistenceError::Unexpected("Database error".to_string())));

        let use_case = GetSettingsUseCase::new(Arc::new(mock_persistence));
        let result = use_case.execute().await;

        assert!(result.is_err());
        match result.unwrap_err() {
            GetSettingsError::PersistenceError(_) => {}
        }
    }
}
