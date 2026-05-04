#[derive(Clone, Default)]
pub struct AuthState {
    auth_token: Option<String>,
    refresh_token: Option<String>,
}

impl AuthState {
    pub fn is_authenticated(&self) -> bool {
        self.auth_token.is_some()
    }

    pub fn auth_token(&self) -> Option<&str> {
        self.auth_token.as_deref()
    }

    pub fn refresh_token(&self) -> Option<&str> {
        self.refresh_token.as_deref()
    }

    pub fn set_auth_token(&mut self, token: Option<String>) {
        self.auth_token = token;
    }

    pub fn delete_auth_token(&mut self) {
        self.auth_token = None;
    }

    pub fn set_refresh_token(&mut self, token: Option<String>) {
        self.refresh_token = token;
    }

    pub fn delete_refresh_token(&mut self) {
        self.refresh_token = None;
    }
}
