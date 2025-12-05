use crate::app_error::AppResult;
use crate::traits::user_setting_persistence::UserSettingPersistence;
use domain::entities::user_setting::UserSetting;
use std::sync::Arc;

pub struct GetSettingsUseCase {
    setting_persistence: Arc<dyn UserSettingPersistence>,
}

impl GetSettingsUseCase {
    pub fn new(setting_persistence: Arc<dyn UserSettingPersistence>) -> Self {
        Self {
            setting_persistence,
        }
    }

    pub async fn execute(&self) -> AppResult<Vec<UserSetting>> {
        self.setting_persistence.find_all().await
    }
}
