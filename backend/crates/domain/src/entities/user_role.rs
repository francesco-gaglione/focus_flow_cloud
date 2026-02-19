use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserRole {
    Admin,
    User,
}

impl Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            UserRole::Admin => write!(f, "admin"),
            UserRole::User => write!(f, "user"),
        }
    }
}

impl From<String> for UserRole {
    fn from(value: String) -> Self {
        match value.as_str() {
            "admin" => UserRole::Admin,
            "user" => UserRole::User,
            _ => UserRole::User,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(UserRole::Admin.to_string(), "admin");
        assert_eq!(UserRole::User.to_string(), "user");
    }

    #[test]
    fn test_from_string() {
        assert_eq!(UserRole::from("admin".to_string()), UserRole::Admin);
        assert_eq!(UserRole::from("user".to_string()), UserRole::User);
        assert_eq!(UserRole::from("invalid".to_string()), UserRole::User); // Default
    }
}
