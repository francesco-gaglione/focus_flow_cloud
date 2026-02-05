use crate::traits::password_policy::{PasswordPolicy, PasswordPolicyError, PasswordPolicyResult};

pub struct PasswordPolicyService;

impl PasswordPolicy for PasswordPolicyService {
    fn validate(&self, password: &str) -> PasswordPolicyResult<()> {
        if password.len() < 8 {
            Err(PasswordPolicyError::InvalidLength(
                "Password must be at least 8 characters long".to_string(),
            ))
        } else if !password.chars().any(char::is_uppercase) {
            Err(PasswordPolicyError::MissingUppercase)
        } else if !password.chars().any(char::is_lowercase) {
            Err(PasswordPolicyError::MissingLowercase)
        } else if !password.chars().any(char::is_numeric) {
            Err(PasswordPolicyError::MissingNumber)
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
