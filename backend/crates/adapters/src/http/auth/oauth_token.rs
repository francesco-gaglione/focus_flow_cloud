use crate::http::app_state::AppState;
use crate::http_error::{HttpError, HttpResult};
use crate::openapi::AUTH_TAG;
use application::user::use_cases::user::login_user::LoginCommand;
use axum::extract::State;
use axum::Form;
use axum::Json;
use secrecy::SecretBox;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct OAuthTokenForm {
    pub username: String,
    pub password: String,
    pub grant_type: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OAuthTokenResponse {
    pub access_token: String,
    pub token_type: String,
}

#[utoipa::path(
    post,
    path = "/api/auth/token",
    tag = AUTH_TAG,
    summary = "OAuth2 password grant (Scalar auth)",
    request_body(
        content = OAuthTokenForm,
        content_type = "application/x-www-form-urlencoded"
    ),
    responses(
        (status = 200, description = "Token issued", body = OAuthTokenResponse),
        (status = 401, description = "Invalid credentials"),
    )
)]
pub async fn oauth_token_api(
    State(state): State<AppState>,
    Form(form): Form<OAuthTokenForm>,
) -> HttpResult<Json<OAuthTokenResponse>> {
    let cmd = LoginCommand {
        username: form.username,
        password: SecretBox::new(form.password.into_boxed_str()),
    };

    let result = state.login_uc.execute(cmd).await.map_err(HttpError::from)?;

    Ok(Json(OAuthTokenResponse {
        access_token: result.token,
        token_type: "bearer".to_string(),
    }))
}
