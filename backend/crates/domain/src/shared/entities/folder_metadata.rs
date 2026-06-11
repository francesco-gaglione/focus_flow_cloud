use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FolderMetadata {
    id: Uuid,
    user_id: Uuid,
    name: String,
    parent_id: Option<Uuid>,
    path: String, // root = "/", children = "/{id}", grandchildren = "/{id}/{id}"
}

impl FolderMetadata {
    pub fn new(name: &str, user_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            name: name.to_string(),
            parent_id: None,
            path: "/".to_string(),
        }
    }

    pub fn reconstitute(
        id: Uuid,
        user_id: Uuid,
        name: String,
        parent_id: Option<Uuid>,
        path: String,
    ) -> Self {
        Self {
            id,
            user_id,
            name,
            parent_id,
            path,
        }
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

    pub fn parent_id(&self) -> Option<Uuid> {
        self.parent_id
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn is_root(&self) -> bool {
        self.parent_id.is_none()
    }

    pub fn depth(&self) -> usize {
        if self.is_root() {
            return 0;
        }
        self.path.matches('/').count()
    }

    pub fn rename(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_is_root() {
        let user_id = Uuid::new_v4();
        let m = FolderMetadata::new("my folder", user_id);

        assert!(m.is_root());
        assert_eq!(m.name(), "my folder");
        assert_eq!(m.user_id(), user_id);
        assert_eq!(m.path(), "/");
        assert_eq!(m.parent_id(), None);
        assert_ne!(m.id(), Uuid::nil());
    }

    #[test]
    fn test_reconstitute_preserves_fields() {
        let id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let parent_id = Uuid::new_v4();
        let path = format!("/{}", parent_id);

        let m = FolderMetadata::reconstitute(
            id,
            user_id,
            "child".to_string(),
            Some(parent_id),
            path.clone(),
        );

        assert_eq!(m.id(), id);
        assert_eq!(m.user_id(), user_id);
        assert_eq!(m.name(), "child");
        assert_eq!(m.parent_id(), Some(parent_id));
        assert_eq!(m.path(), path);
        assert!(!m.is_root());
    }

    #[test]
    fn test_depth_root() {
        let m = FolderMetadata::new("root", Uuid::new_v4());
        assert_eq!(m.depth(), 0);
    }

    #[test]
    fn test_depth_child() {
        let child_id = Uuid::new_v4();
        let m = FolderMetadata::reconstitute(
            child_id,
            Uuid::new_v4(),
            "child".to_string(),
            Some(Uuid::new_v4()),
            format!("/{}", child_id),
        );
        assert_eq!(m.depth(), 1);
    }

    #[test]
    fn test_depth_grandchild() {
        let parent_id = Uuid::new_v4();
        let child_id = Uuid::new_v4();
        let m = FolderMetadata::reconstitute(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "grandchild".to_string(),
            Some(child_id),
            format!("/{}/{}", parent_id, child_id),
        );
        assert_eq!(m.depth(), 2);
    }

    #[test]
    fn test_rename() {
        let mut m = FolderMetadata::new("old", Uuid::new_v4());
        m.rename("new");
        assert_eq!(m.name(), "new");
    }
}
