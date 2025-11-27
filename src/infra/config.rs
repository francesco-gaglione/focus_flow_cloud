#[derive(Clone)]
pub struct AppConfig {
    pub server_port: u16,
    pub cors_origin: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let server_port = std::env::var("SERVER_PORT").expect("SERVER_PORT must be set");
        let cors_origin = std::env::var("CORS_ORIGIN").expect("CORS_ORIGIN must be set");

        Self {
            server_port: server_port.parse().expect("SERVER_PORT must be a number"),
            cors_origin,
        }
    }
}
