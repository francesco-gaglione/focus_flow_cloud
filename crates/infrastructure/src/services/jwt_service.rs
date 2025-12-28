use async_trait::async_trait;
use chrono::{Duration, Utc};
use domain::entities::user::User;
use domain::error::domain_error::{DomainError, DomainResult};
use domain::services::token_service::TokenService;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
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
    fn generate_token(&self, user: &User) -> DomainResult<String> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user.id().to_string(),
            exp: expiration as usize,
            iat: Utc::now().timestamp() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| DomainError::TokenGenerationError(e.to_string()))
    }

    fn verify_token(&self, token: &str) -> DomainResult<String> {
        let validation = jsonwebtoken::Validation::default();
        // validation.validate_exp = true; // Default is true

        jsonwebtoken::decode::<Claims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        )
        .map(|data| data.claims.sub)
        .map_err(|e| DomainError::TokenVerificationError(format!("Invalid token: {}", e)))
    }
}
