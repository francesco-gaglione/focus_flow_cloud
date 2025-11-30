use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Category {
    id: Uuid,
    name: String,
    description: Option<String>,
    color: String,
}

impl Category {
    pub fn new(id: Uuid, name: String, description: Option<String>, color: String) -> Self {
        Category {
            id,
            name,
            description,
            color,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
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
}

#[cfg(test)]
mod category_tests {
    use super::*;

    #[test]
    fn test_new_category() {
        let id = Uuid::new_v4();
        let name = "Test Category".to_string();
        let description = Some("This is a test category".to_string());
        let color = "#FF0000".to_string();

        let category = Category::new(id, name, description, color);

        assert_eq!(category.id(), id);
        assert_eq!(category.name(), "Test Category");
        assert_eq!(category.description(), Some("This is a test category"));
        assert_eq!(category.color(), "#FF0000");
    }
}
