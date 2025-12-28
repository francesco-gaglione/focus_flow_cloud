use crate::http::app_state::AppState;
use crate::http::model::session_model::UserSession;
use crate::http_error::HttpError;
use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::Response,
};
use serde::Deserialize;
use tracing::instrument;

#[derive(Deserialize)]
struct AuthQuery {
    token: Option<String>,
}

#[instrument(skip_all, name = "auth_middleware")]
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, HttpError> {
    let token = match req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
    {
        Some(t) => t.to_string(),
        None => {
            let query = req.uri().query().unwrap_or("");
            let query_params: AuthQuery =
                serde_urlencoded::from_str(query).unwrap_or(AuthQuery { token: None });
            query_params.token.ok_or(HttpError::Unauthorized(
                "Missing Authorization header or token query param".to_string(),
            ))?
        }
    };

    let user_id_str = state
        .token_service
        .verify_token(&token)
        .map_err(|_| HttpError::Unauthorized("Invalid token".to_string()))?;

    let user_id = uuid::Uuid::parse_str(&user_id_str)
        .map_err(|_| HttpError::Unauthorized("Invalid user ID in token".to_string()))?;

    let session = UserSession { user_id };
    req.extensions_mut().insert(session);

    Ok(next.run(req).await)
}
