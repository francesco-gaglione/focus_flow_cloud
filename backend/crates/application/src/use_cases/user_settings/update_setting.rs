use crate::persistence_traits::persistence_error::PersistenceError;
use crate::persistence_traits::user_setting_persistence::UserSettingPersistence;
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error, PartialEq)]
pub enum UpdateSettingsError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type UpdateSettingsResult<T> = Result<T, UpdateSettingsError>;

pub struct UpdateSettingUseCase {
    setting_persistence: Arc<dyn UserSettingPersistence>,
}

impl UpdateSettingUseCase {
    pub fn new(setting_persistence: Arc<dyn UserSettingPersistence>) -> Self {
        Self {
            setting_persistence,
        }
    }

    pub async fn execute(
        &self,
        user_id: Uuid,
        key: String,
        value: String,
    ) -> UpdateSettingsResult<()> {
        let settings = self.setting_persistence.find_all().await?;

        if settings.iter().any(|s| s.key() == key) {
            self.setting_persistence
                .update_setting(user_id, key, value)
                .await?;
        } else {
            self.setting_persistence
                .create_setting(user_id, key, value)
                .await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence_traits::user_setting_persistence::MockUserSettingPersistence;
    use domain::entities::user_setting::UserSetting;

    #[tokio::test]
    async fn test_update_setting_existing() {
        let mut mock_persistence = MockUserSettingPersistence::new();
        let user_id = Uuid::new_v4();
        let key = "theme".to_string();
        let value = "light".to_string();

        let existing_settings = vec![UserSetting::new(key.clone(), Some("dark".to_string()))];

        mock_persistence
            .expect_find_all()
            .returning(move || Ok(existing_settings.clone()));

        mock_persistence
            .expect_update_setting()
            .with(
                mockall::predicate::eq(user_id),
                mockall::predicate::eq(key.clone()),
                mockall::predicate::eq(value.clone()),
            )
            .returning(|_, _, _| Ok(()));

        let use_case = UpdateSettingUseCase::new(Arc::new(mock_persistence));
        let result = use_case.execute(user_id, key, value).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_setting_new() {
        let mut mock_persistence = MockUserSettingPersistence::new();
        let user_id = Uuid::new_v4();
        let key = "language".to_string();
        let value = "en".to_string();

        mock_persistence.expect_find_all().returning(|| Ok(vec![]));

        mock_persistence
            .expect_create_setting()
            .with(
                mockall::predicate::eq(user_id),
                mockall::predicate::eq(key.clone()),
                mockall::predicate::eq(value.clone()),
            )
            .returning(|_, _, _| Ok(()));

        let use_case = UpdateSettingUseCase::new(Arc::new(mock_persistence));
        let result = use_case.execute(user_id, key, value).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_setting_persistence_error() {
        let mut mock_persistence = MockUserSettingPersistence::new();
        mock_persistence
            .expect_find_all()
            .returning(|| Err(PersistenceError::Unexpected("DB Error".to_string())));

        let use_case = UpdateSettingUseCase::new(Arc::new(mock_persistence));
        let result = use_case
            .execute(Uuid::new_v4(), "key".to_string(), "val".to_string())
            .await;

        assert!(result.is_err());
    }
}
