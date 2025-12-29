use crate::{
    error::domain_error::{DomainError, DomainResult},
    traits::password_policy::PasswordPolicy,
};

pub struct PasswordPolicyService;

impl PasswordPolicy for PasswordPolicyService {
    fn validate(&self, password: &str) -> DomainResult<()> {
        if password.len() < 8 {
            Err(DomainError::InvalidPasswordPolicy(
                "Password must be at least 8 characters long".to_string(),
            ))
        } else if !password.chars().any(char::is_uppercase) {
            Err(DomainError::InvalidPasswordPolicy(
                "Password must contain at least one uppercase letter".to_string(),
            ))
        } else if !password.chars().any(char::is_lowercase) {
            Err(DomainError::InvalidPasswordPolicy(
                "Password must contain at least one lowercase letter".to_string(),
            ))
        } else if !password.chars().any(char::is_numeric) {
            Err(DomainError::InvalidPasswordPolicy(
                "Password must contain at least one number".to_string(),
            ))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_password() {
        let service = PasswordPolicyService;
        assert!(service.validate("Password1").is_ok());
        assert!(service.validate("password").is_err());
        assert!(service.validate("PASSWORD").is_err());
        assert!(service.validate("password1").is_err());
    }
}
