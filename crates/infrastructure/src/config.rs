#[derive(Clone)]
pub struct AppConfig {
    pub server_port: u16,
    pub cors_origin: String,
    pub database_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let server_port = std::env::var("SERVER_PORT").expect("SERVER_PORT must be set");
        let cors_origin = std::env::var("CORS_ORIGIN").expect("CORS_ORIGIN must be set");
        let db_base_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let postgres_user = std::env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
        let postgres_password =
            std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
        let postgres_db = std::env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");

        let database_url = format!(
            "postgres://{}:{}@{}/{}",
            postgres_user, postgres_password, db_base_url, postgres_db
        );

        Self {
            server_port: server_port.parse().expect("SERVER_PORT must be a number"),
            cors_origin,
            database_url,
        }
    }
}
