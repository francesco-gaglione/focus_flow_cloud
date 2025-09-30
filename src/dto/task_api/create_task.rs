use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskDto {
    #[validate(custom(function = "validate_uuid"))]
    pub category_id: Option<String>,

    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: String,

    #[validate(length(max = 255, message = "Description must not exceed 255 characters"))]
    pub description: Option<String>,

    pub scheduled_date: Option<i64>, //timestamp in seconds
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateTaskResponseDto {
    pub id: String,
}

// Custom validator per UUID
fn validate_uuid(uuid_str: &str) -> Result<(), ValidationError> {
    Uuid::parse_str(uuid_str).map(|_| ()).map_err(|_| {
        let mut error = ValidationError::new("invalid_uuid");
        error.message = Some("Category ID must be a valid UUID".into());
        error
    })
}
