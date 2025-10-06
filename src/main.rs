use focus_flow_cloud::infra::app::create_app;
use focus_flow_cloud::infra::setup::init_app_state;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let app_state = init_app_state().await?;

    let app = create_app(app_state.clone());

    let listener =
        tokio::net::TcpListener::bind(format!("127.0.0.1:{}", app_state.config.server_port))
            .await
            .unwrap();

    info!("Backend listening at {}", &listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
