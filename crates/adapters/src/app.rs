use crate::http::{app_state::AppState, request_id::RequestId};
use crate::middleware::request_id_middleware::RequestIdLayer;
use crate::{
    http::routes::{api_routes, ws_routes},
    openapi::ApiDoc,
};
use axum::Router;
use http::{header, Method};
use time::Duration;
use tower::ServiceBuilder;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::trace::TraceLayer;
use tower_sessions::{MemoryStore, SessionManagerLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use uuid::Uuid;

pub fn create_app(app_state: AppState) -> Router {
    let session_store = MemoryStore::default();

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(tower_sessions::Expiry::OnInactivity(Duration::hours(24))); // 24 hours

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/api", api_routes(app_state.clone()))
        .nest("/ws", ws_routes(app_state.clone()))
        .with_state(app_state.clone())
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &http::Request<_>| {
                let request_id = request
                    .extensions()
                    .get::<RequestId>()
                    .map(|id| id.0)
                    .unwrap_or_else(Uuid::new_v4);
                tracing::info_span!(
                    "http-request",
                    method = %request.method(),
                    uri = %request.uri(),
                    version = ?request.version(),
                    request_id = %request_id
                )
            }),
        )
        .layer(ServiceBuilder::new().layer(RequestIdLayer))
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
        .layer(session_layer)
}
