use thiserror::Error;
use uuid::Uuid;

use crate::helpers::validate_hex_color;

#[derive(Debug, Error)]
pub enum CategoryError {
    #[error("Invalid color: {0}")]
    InvalidColor(String),
}

pub type CategoryResult<T> = Result<T, CategoryError>;

#[derive(Debug, Clone, PartialEq)]
pub struct Category {
    id: Uuid,
    user_id: Uuid,
    name: String,
    description: Option<String>,
    color: String,
}

impl Category {
    pub fn create(
        user_id: Uuid,
        name: String,
        description: Option<String>,
        color: String,
    ) -> CategoryResult<Self> {
        if !validate_hex_color(&color) {
            return Err(CategoryError::InvalidColor(color));
        }

        Ok(Category {
            id: Uuid::new_v4(),
            user_id,
            name,
            description,
            color,
        })
    }

    pub fn reconstitute(
        id: Uuid,
        user_id: Uuid,
        name: String,
        description: Option<String>,
        color: String,
    ) -> CategoryResult<Self> {
        if !validate_hex_color(&color) {
            return Err(CategoryError::InvalidColor(color));
        }

        Ok(Category {
            id,
            user_id,
            name,
            description,
            color,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn color(&self) -> &str {
        &self.color
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn update_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    pub fn update_color(&mut self, color: String) {
        self.color = color;
    }
}

#[cfg(test)]
mod category_tests {
    use super::*;

    #[test]
    fn test_new_category_valid() {
        let user_id = Uuid::new_v4();
        let name = "Test Category".to_string();
        let description = Some("This is a test category".to_string());
        let color = "#FF0000".to_string();

        let result = Category::create(user_id, name.clone(), description.clone(), color.clone());

        assert!(result.is_ok());
        let category = result.unwrap();
        assert_eq!(category.user_id(), user_id);
        assert_eq!(category.name(), name);
        assert_eq!(category.description(), description.as_deref());
        assert_eq!(category.color(), color);
    }

    #[test]
    fn test_new_category_invalid_color() {
        let user_id = Uuid::new_v4();
        let name = "Test Category".to_string();
        let color = "invalid-color".to_string();

        let result = Category::create(user_id, name, None, color);

        assert!(matches!(result, Err(CategoryError::InvalidColor(_))));
    }

    #[test]
    fn test_reconstitute_valid() {
        let id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let name = "Reconstituted Category".to_string();
        let description = None;
        let color = "#00FF00".to_string();

        let result = Category::reconstitute(id, user_id, name.clone(), description, color.clone());

        assert!(result.is_ok());
        let category = result.unwrap();
        assert_eq!(category.id(), id);
        assert_eq!(category.name(), name);
        assert_eq!(category.color(), color);
    }

    #[test]
    fn test_reconstitute_invalid_color() {
        let id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let name = "Invalid Color Category".to_string();
        let color = "not-a-hex-code".to_string();

        let result = Category::reconstitute(id, user_id, name, None, color);

        assert!(matches!(result, Err(CategoryError::InvalidColor(_))));
    }

    #[test]
    fn test_update_category_fields() {
        let user_id = Uuid::new_v4();
        let mut category = Category::create(
            user_id,
            "Old Name".to_string(),
            Some("Old Desc".to_string()),
            "#000000".to_string(),
        )
        .unwrap();

        let new_name = "New Name".to_string();
        category.update_name(new_name.clone());
        assert_eq!(category.name(), new_name);

        let new_desc = Some("New Desc".to_string());
        category.update_description(new_desc.clone());
        assert_eq!(category.description(), new_desc.as_deref());

        let new_color = "#FFFFFF".to_string();
        category.update_color(new_color.clone());
        assert_eq!(category.color(), new_color);
    }
}
