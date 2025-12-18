use crate::error::domain_error::DomainResult;

#[cfg_attr(test, mockall::automock)]
pub trait PasswordHasher: Send + Sync {
    fn hash_password(&self, password: &str) -> DomainResult<String>;

    fn verify_password(&self, password: &str, hashed_password: &str) -> DomainResult<bool>;
}
