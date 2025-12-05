use crate::app_error::AppResult;
use domain::traits::user_setting_persistence::UserSettingPersistence;
use std::sync::Arc;

pub struct UpdateSettingUseCase {
    setting_persistence: Arc<dyn UserSettingPersistence>,
}

impl UpdateSettingUseCase {
    pub fn new(setting_persistence: Arc<dyn UserSettingPersistence>) -> Self {
        Self {
            setting_persistence,
        }
    }

    pub async fn execute(&self, key: String, value: String) -> AppResult<()> {
        let settings = self.setting_persistence.find_all().await?;

        if settings.iter().any(|s| s.key() == key) {
            self.setting_persistence.update_setting(key, value).await?;
        } else {
            self.setting_persistence.create_setting(key, value).await?;
        }

        Ok(())
    }
}
