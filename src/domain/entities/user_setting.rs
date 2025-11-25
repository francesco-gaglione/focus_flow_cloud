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
