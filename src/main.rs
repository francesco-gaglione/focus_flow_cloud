use focus_flow_cloud::infra::{
    app::create_app,
    middleware::RequestIdLayer,
    setup::{init_app_state, init_tracing},
};
use tower::ServiceBuilder;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    init_tracing();

    let app_state = init_app_state().await?;

    let app = create_app(app_state.clone()).layer(ServiceBuilder::new().layer(RequestIdLayer));

    let listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{}", app_state.config.server_port))
            .await
            .unwrap();

    info!("Backend listening at {}", &listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
