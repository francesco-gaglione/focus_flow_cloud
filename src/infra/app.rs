use crate::adapters::http::{app_state::AppState, request_id::RequestId};
use crate::adapters::{
    http::routes::{api_routes, ws_routes},
    openapi::ApiDoc,
};
use axum::Router;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use uuid::Uuid;

pub fn create_app(app_state: AppState) -> Router {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/api", api_routes())
        .nest("/ws", ws_routes())
        .with_state(app_state)
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
}
