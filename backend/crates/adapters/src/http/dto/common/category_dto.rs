use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CategoryDto {
    pub id: String,
    pub name: String,
    pub color: String,
}
