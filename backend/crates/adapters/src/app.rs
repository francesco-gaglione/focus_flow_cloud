use crate::shared::http::app_state::AppState;
use crate::shared::http::request_id::RequestId;
use crate::middleware::request_id_middleware::RequestIdLayer;
use crate::shared::http::routes::{api_routes, ws_routes};
use crate::openapi::ApiDoc;
use axum::Router;
use http::{header, Method};
use time::Duration;
use tower::ServiceBuilder;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tower_sessions::{MemoryStore, SessionManagerLayer};
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};
use utoipa_swagger_ui::SwaggerUi;
use uuid::Uuid;

pub fn create_app(app_state: AppState) -> Router {
    let session_store = MemoryStore::default();

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(tower_sessions::Expiry::OnInactivity(Duration::hours(24))); // 24 hours

    let mut router = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
        .nest("/api", api_routes(app_state.clone()))
        .nest("/ws", ws_routes(app_state.clone()))
        .with_state(app_state.clone());

    if let Ok(dist_dir) = std::env::var("PWA_DIST_DIR") {
        let index = ServeFile::new(format!("{}/index.html", dist_dir));
        router = router.fallback_service(ServeDir::new(dist_dir).fallback(index));
    }

    router
        .layer(session_layer)
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::mirror_request())
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::DELETE,
                    Method::OPTIONS,
                    Method::HEAD,
                    Method::PATCH,
                ])
                .allow_headers([header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
                .allow_credentials(true),
        )
        .layer(ServiceBuilder::new().layer(RequestIdLayer))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &http::Request<_>| {
                    let request_id = request
                        .extensions()
                        .get::<RequestId>()
                        .map(|id| id.0)
                        .unwrap_or_else(Uuid::new_v4);
                    let path = request.uri().path();
                    let method = request.method();
                    tracing::info_span!(
                        "http_request",
                        otel.name = format!("{method} {path}"),
                        http.method = %method,
                        http.target = %path,
                        http.status_code = tracing::field::Empty,
                        request_id = %request_id,
                        user_id = tracing::field::Empty,
                    )
                })
                .on_response(
                    tower_http::trace::DefaultOnResponse::new()
                        .level(tracing::Level::INFO)
                        .include_headers(false),
                ),
        )
}
