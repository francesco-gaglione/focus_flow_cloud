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
    fn test_new_category() {
        let id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let name = "Test Category".to_string();
        let description = Some("This is a test category".to_string());
        let color = "#FF0000".to_string();

        let category = Category::reconstitute(id, user_id, name, description, color).unwrap();

        assert_eq!(category.id(), id);
        assert_eq!(category.user_id(), user_id);
        assert_eq!(category.name(), "Test Category");
        assert_eq!(category.description(), Some("This is a test category"));
        assert_eq!(category.color(), "#FF0000");
    }
}
