use async_trait::async_trait;
use chrono::{Duration, Utc};
use domain::services::token_service::{TokenService, TokenServiceError};
use domain::{entities::user::User, services::token_service::TokenServiceResult};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
    typ: String,
}

pub struct JwtService {
    secret: String,
}

impl JwtService {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}

#[async_trait]
impl TokenService for JwtService {
    fn generate_token(&self, user: &User) -> TokenServiceResult<String> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(1))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user.id().to_string(),
            exp: expiration as usize,
            iat: Utc::now().timestamp() as usize,
            typ: "access".to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| TokenServiceError::TokenGenerationError(e.to_string()))
    }

    fn generate_refresh_token(&self, user: &User) -> TokenServiceResult<String> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::days(7))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user.id().to_string(),
            exp: expiration as usize,
            iat: Utc::now().timestamp() as usize,
            typ: "refresh".to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| TokenServiceError::TokenGenerationError(e.to_string()))
    }

    fn verify_token(&self, token: &str) -> TokenServiceResult<String> {
        let validation = jsonwebtoken::Validation::default();

        let claims = jsonwebtoken::decode::<Claims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        )
        .map(|data| data.claims)
        .map_err(|e| TokenServiceError::InvalidToken)?;

        if claims.typ != "access" {
            return Err(TokenServiceError::InvalidToken);
        }

        Ok(claims.sub)
    }

    fn verify_refresh_token(&self, token: &str) -> TokenServiceResult<String> {
        let validation = jsonwebtoken::Validation::default();

        let claims = jsonwebtoken::decode::<Claims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        )
        .map(|data| data.claims)
        .map_err(|e| TokenServiceError::InvalidToken)?;

        if claims.typ != "refresh" {
            return Err(TokenServiceError::InvalidToken);
        }

        Ok(claims.sub)
    }
}
