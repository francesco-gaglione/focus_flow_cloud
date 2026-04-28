use crate::http_error::HttpResult;
use crate::openapi::AUTH_TAG;
use axum::Json;
use shared::auth::LogoutResponseDto;
use tower_sessions::Session;

#[utoipa::path(
    post,
    path = "/api/auth/logout",
    tag = AUTH_TAG,
    summary = "Logout user",
    responses(
        (status = 200, description = "Logout successful", body = LogoutResponseDto),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn logout_api(session: Session) -> HttpResult<Json<LogoutResponseDto>> {
    session.clear().await;

    Ok(Json(LogoutResponseDto {
        message: "Logout successful".to_string(),
    }))
}
