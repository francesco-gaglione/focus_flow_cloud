use focus_flow_cloud::infra::{
    app::create_app,
    config::AppConfig,
    setup::{init_app_state, init_tracing},
};
use std::sync::Once;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;
use tokio::net::TcpListener;

static TRACING: Once = Once::new();

pub struct TestContext {
    pub base_url: String,
    #[allow(dead_code)] // Keep container alive
    pub container: testcontainers::ContainerAsync<Postgres>,
}

pub async fn setup() -> TestContext {
    // Initialize tracing once
    TRACING.call_once(|| {
        init_tracing();
    });

    // Start Postgres container
    let container = Postgres::default().start().await.unwrap();
    let host_ip = container.get_host().await.unwrap();
    let host_port = container.get_host_port_ipv4(5432).await.unwrap();
    let db_url = format!("postgres://postgres:postgres@{host_ip}:{host_port}/postgres");

    // Setup AppConfig
    let config = AppConfig {
        server_port: 0,
        cors_origin: "*".to_string(),
        database_url: db_url,
    };

    // Initialize App State (this runs migrations)
    let app_state = init_app_state(config).await.unwrap();

    // Create App Router
    let app = create_app(app_state);

    // Start Server
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    TestContext {
        base_url: format!("http://127.0.0.1:{}", port),
        container,
    }
}
