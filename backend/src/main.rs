use adapters::app::create_app;
use infrastructure::setup::{init_app_state, init_tracing};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    init_tracing();

    let config = infrastructure::config::load_from_env();
    let app_state = init_app_state(config).await?;

    let app = create_app(app_state.clone());

    let listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{}", app_state.config.server_port))
            .await
            .unwrap();

    info!("Backend listening at {}", &listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
