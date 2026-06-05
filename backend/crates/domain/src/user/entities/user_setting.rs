#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserSetting {
    key: String,
    value: Option<String>,
}

impl UserSetting {
    pub fn new(key: String, value: Option<String>) -> Self {
        Self { key, value }
    }

    pub fn update_value(&mut self, value: String) {
        self.value = Some(value);
    }

    pub fn update_key(&mut self, key: String) {
        self.key = key;
    }

    pub fn is_empty(&self) -> bool {
        self.value().is_none()
    }

    pub fn key(&self) -> String {
        self.key.clone()
    }

    pub fn value(&self) -> Option<String> {
        self.value.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_user_setting() {
        let key = "theme".to_string();
        let value = Some("dark".to_string());

        let setting = UserSetting::new(key.clone(), value.clone());

        assert_eq!(setting.key(), key);
        assert_eq!(setting.value(), value);
        assert!(!setting.is_empty());
    }

    #[test]
    fn test_empty_user_setting() {
        let key = "language".to_string();

        let setting = UserSetting::new(key.clone(), None);

        assert_eq!(setting.key(), key);
        assert!(setting.value().is_none());
        assert!(setting.is_empty());
    }

    #[test]
    fn test_update_user_setting() {
        let key = "notifications".to_string();
        let mut setting = UserSetting::new(key.clone(), None);

        assert!(setting.is_empty());

        let new_value = "enabled".to_string();
        setting.update_value(new_value.clone());

        assert_eq!(setting.value(), Some(new_value));
        assert!(!setting.is_empty());

        let new_key = "email_notifications".to_string();
        setting.update_key(new_key.clone());

        assert_eq!(setting.key(), new_key);
    }
}
