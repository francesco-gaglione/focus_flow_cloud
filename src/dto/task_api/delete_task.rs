use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeleteTasksDto {
    #[validate(custom(function = "validate_uuid"))]
    pub task_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteTasksResponseDto {
    pub deleted_ids: Vec<String>,
}

// Custom validator per UUID
fn validate_uuid(uuids_str: &Vec<String>) -> Result<(), ValidationError> {
    for id in uuids_str {
        Uuid::parse_str(id).map(|_| ()).map_err(|_| {
            let mut error = ValidationError::new("invalid_uuid");
            error.message = Some("Category ID must be a valid UUID".into());
            error
        })?
    }
    Ok(())
}
