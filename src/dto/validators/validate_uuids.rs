use uuid::Uuid;
use validator::ValidationError;

// Custom validator per UUID
pub fn validate_uuids(uuids_str: &Vec<String>) -> Result<(), ValidationError> {
    for id in uuids_str {
        Uuid::parse_str(id).map(|_| ()).map_err(|_| {
            let mut error = ValidationError::new("invalid_uuid");
            error.message = Some("Category ID must be a valid UUID".into());
            error
        })?
    }
    Ok(())
}
