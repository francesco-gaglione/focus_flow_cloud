use crate::http::app_state::AppState;
use crate::http::model::session_model::UserSession;
use crate::http_error::HttpError;
use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::Response,
};
use tracing::instrument;

#[instrument(skip_all, name = "auth_middleware")]
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, HttpError> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(HttpError::Unauthorized("Missing Authorization header".to_string()))?;

    let token = if auth_header.starts_with("Bearer ") {
        &auth_header[7..]
    } else {
        return Err(HttpError::Unauthorized(
            "Invalid Authorization header format".to_string(),
        ));
    };

    let user_id_str = state
        .token_service
        .verify_token(token)
        .map_err(|_| HttpError::Unauthorized("Invalid token".to_string()))?;

    let user_id = uuid::Uuid::parse_str(&user_id_str)
        .map_err(|_| HttpError::Unauthorized("Invalid user ID in token".to_string()))?;

    let session = UserSession { user_id };
    req.extensions_mut().insert(session);

    Ok(next.run(req).await)
}
