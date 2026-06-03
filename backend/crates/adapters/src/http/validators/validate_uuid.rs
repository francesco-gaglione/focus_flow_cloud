use uuid::Uuid;
use validator::ValidationError;

pub fn validate_uuid(uuid_str: &str) -> Result<(), ValidationError> {
    Uuid::parse_str(uuid_str).map(|_| ()).map_err(|_| {
        let mut error = ValidationError::new("invalid_uuid");
        error.message = Some("ID must be a valid UUID".into());
        error
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_uuid() {
        let id = uuid::Uuid::new_v4().to_string();
        assert!(validate_uuid(&id).is_ok());
    }

    #[test]
    fn test_invalid_uuid_random_string() {
        assert!(validate_uuid("not-a-uuid").is_err());
    }

    #[test]
    fn test_invalid_uuid_empty() {
        assert!(validate_uuid("").is_err());
    }

    #[test]
    fn test_error_code() {
        let err = validate_uuid("bad").unwrap_err();
        assert_eq!(err.code, "invalid_uuid");
    }
}
