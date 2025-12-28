use adapters::config::AppConfig;

pub fn load_from_env() -> AppConfig {
    let server_port = std::env::var("SERVER_PORT").expect("SERVER_PORT must be set");
    let cors_origin = std::env::var("CORS_ORIGIN").expect("CORS_ORIGIN must be set");
    let db_base_url =
        std::env::var("DATABASE_BASE_URL").expect("DATABASE_BASE_URL must be set");
    let postgres_user = std::env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let postgres_password =
        std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let postgres_db = std::env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let admin_username = std::env::var("ADMIN_USERNAME").ok();
    let admin_password = std::env::var("ADMIN_PASSWORD").ok();

    let database_url = format!(
        "postgres://{}:{}@{}/{}",
        postgres_user, postgres_password, db_base_url, postgres_db
    );

    AppConfig {
        server_port: server_port.parse().expect("SERVER_PORT must be a number"),
        cors_origin,
        database_url,
        jwt_secret,
        admin_username,
        admin_password,
    }
}
