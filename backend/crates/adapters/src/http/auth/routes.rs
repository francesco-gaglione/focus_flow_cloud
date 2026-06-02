use crate::http::app_state::AppState;
use crate::http::auth::login::login_api;
use crate::http::auth::logout::logout_api;
use crate::http::auth::oauth_token::oauth_token_api;
use crate::http::auth::refresh::refresh_api;
use axum::routing::post;
use axum::Router;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login_api))
        .route("/token", post(oauth_token_api))
        .route("/refresh", post(refresh_api))
        .route("/logout", post(logout_api))
}
