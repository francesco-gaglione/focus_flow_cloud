use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct AppConfig {
    pub server_port: u16,
    pub cors_origin: String,
    pub database_url: String,
    pub jwt_secret: String,
    pub admin_username: Option<String>,
    pub admin_password: Option<String>,
}
