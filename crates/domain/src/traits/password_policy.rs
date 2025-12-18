use crate::error::domain_error::DomainResult;

pub trait PasswordPolicy: Send + Sync {
    fn validate(&self, password: &str) -> DomainResult<()>;
}
