use domain::entities::user_setting::UserSetting;
use domain::error::persistence_error::PersistenceError;
use domain::traits::user_setting_persistence::UserSettingPersistence;
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

impl GetSettingsUseCase {
    pub fn new(setting_persistence: Arc<dyn UserSettingPersistence>) -> Self {
        Self {
            setting_persistence,
        }
    }

    pub async fn execute(&self) -> GetSettingsResult<Vec<UserSetting>> {
        Ok(self.setting_persistence.find_all().await?)
    }
}

//TODO implements tests
