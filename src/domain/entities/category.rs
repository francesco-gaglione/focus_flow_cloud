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
