use crate::http::app_state::AppState;
use axum::{extract::State, response::Json};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct VersionResponse {
    pub version: String,
}

#[utoipa::path(
    get,
    path = "/api/version",
    tag = "System",
    responses(
        (status = 200, description = "Application version", body = VersionResponse)
    )
)]
pub async fn get_version(State(state): State<AppState>) -> Json<VersionResponse> {
    Json(VersionResponse {
        version: state.version.clone(),
    })
}

pub fn router() -> axum::Router<AppState> {
    axum::Router::new().route("/", axum::routing::get(get_version))
}
