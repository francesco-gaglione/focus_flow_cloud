use serde::{Deserialize, Serialize};
use validator::Validate;
use lazy_static::lazy_static;
use regex::Regex;
use utoipa::ToSchema;

lazy_static! {
    static ref COLOR_REGEX: Regex = Regex::new(r"^#[0-9A-Fa-f]{6}$").unwrap();
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateCategoryDto {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(max = 255))]
    pub description: Option<String>,
    #[validate(regex(path = *COLOR_REGEX, message = "Color must be a valid hex code"))]
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateCategoryResponseDto {
    pub id: String,
}
