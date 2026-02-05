use domain::{
    error::persistence_error::PersistenceError,
    traits::user_setting_persistence::UserSettingPersistence,
};
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

//TODO implements tests
