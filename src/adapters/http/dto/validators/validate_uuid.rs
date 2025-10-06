use uuid::Uuid;
use validator::ValidationError;

// Custom validator per UUID
pub fn validate_uuid(uuid_str: &str) -> Result<(), ValidationError> {
    Uuid::parse_str(uuid_str).map(|_| ()).map_err(|_| {
        let mut error = ValidationError::new("invalid_uuid");
        error.message = Some("Category ID must be a valid UUID".into());
        error
    })
}
