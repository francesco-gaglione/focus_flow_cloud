#[derive(Clone)]
pub struct AppConfig {
    pub server_port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let server_port = std::env::var("SERVER_PORT").expect("SERVER_PORT must be set");

        Self {
            server_port: server_port.parse().expect("SERVER_PORT must be a number"),
        }
    }
}
