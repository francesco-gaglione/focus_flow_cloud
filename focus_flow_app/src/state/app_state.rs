#[derive(Clone, Default)]
pub struct AppState {
    server_url: Option<String>,
}

impl AppState {
    pub fn server_url(&self) -> Option<&str> {
        self.server_url.as_deref()
    }

    pub fn set_server_url(&mut self, server_url: String) {
        self.server_url = Some(server_url);
    }

    pub fn is_server_url_set(&self) -> bool {
        self.server_url.is_some()
    }

    pub fn clear_server_url(&mut self) {
        self.server_url = None;
    }
}
