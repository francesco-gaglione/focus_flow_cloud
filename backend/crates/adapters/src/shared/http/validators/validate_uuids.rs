use tracing::warn;
use uuid::Uuid;
use validator::ValidationError;

pub fn validate_uuids(uuids_str: &Vec<String>) -> Result<(), ValidationError> {
    for id in uuids_str {
        Uuid::parse_str(id).map(|_| ()).map_err(|_| {
            let mut error = ValidationError::new("invalid_uuid");
            error.message = Some("Category ID must be a valid UUID".into());
            warn!("Invalid UUID: {}", id);
            error
        })?
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_list_is_ok() {
        assert!(validate_uuids(&vec![]).is_ok());
    }

    #[test]
    fn test_all_valid_uuids() {
        let ids = vec![Uuid::new_v4().to_string(), Uuid::new_v4().to_string()];
        assert!(validate_uuids(&ids).is_ok());
    }

    #[test]
    fn test_one_invalid_uuid_fails() {
        let ids = vec![Uuid::new_v4().to_string(), "not-a-uuid".to_string()];
        assert!(validate_uuids(&ids).is_err());
    }

    #[test]
    fn test_error_code() {
        let err = validate_uuids(&vec!["bad".to_string()]).unwrap_err();
        assert_eq!(err.code, "invalid_uuid");
    }
}
