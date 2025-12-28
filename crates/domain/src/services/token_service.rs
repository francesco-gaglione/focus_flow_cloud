use crate::entities::user::User;
use crate::error::domain_error::DomainResult;
use async_trait::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait TokenService: Send + Sync {
    fn generate_token(&self, user: &User) -> DomainResult<String>;
    fn generate_refresh_token(&self, user: &User) -> DomainResult<String>;
    fn verify_token(&self, token: &str) -> DomainResult<String>;
    fn verify_refresh_token(&self, token: &str) -> DomainResult<String>;
}
