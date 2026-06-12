use uuid::Uuid;

pub struct Folder<T> {
    id: Uuid,
    user_id: Uuid,
    name: String,
    parent_id: Option<Uuid>,
    path: String, // root = "/", children = "/{id}", grandchildren = "/{id}/{id}"
    file: Vec<T>,
}

impl<T> Folder<T> {
    pub fn new(name: &str, user_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            name: name.to_string(),
            parent_id: None,
            path: "/".to_string(),
            file: Vec::new(),
        }
    }

    pub fn new_child(name: &str, user_id: Uuid, parent: &Folder<T>) -> Self {
        let id = Uuid::new_v4();
        let path = if parent.is_root() {
            format!("/{}", id)
        } else {
            format!("{}/{}", parent.path, id)
        };
        Self {
            id,
            user_id,
            name: name.to_string(),
            parent_id: Some(parent.id),
            path,
            file: Vec::new(),
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

    pub fn files(&self) -> &[T] {
        &self.file
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

    pub fn add_file(&mut self, file: T) {
        self.file.push(file);
    }

    pub fn is_ancestor_of(&self, other: &Folder<T>) -> bool {
        if self.id == other.id {
            return false;
        }
        if self.is_root() {
            return true;
        }
        other.path.starts_with(&format!("{}/", self.path))
    }

    // Returns false if: moving into itself, moving into a descendant, or moving root
    pub fn can_move_into(&self, target: &Folder<T>) -> bool {
        if self.is_root() {
            return false;
        }
        if self.id == target.id {
            return false;
        }
        !self.is_ancestor_of(target)
    }
}

#[cfg(test)]
pub mod tests_flashcards_folder {
    use crate::shared::entities::folder::Folder;
    use uuid::Uuid;

    fn make_root(user_id: Uuid) -> Folder<String> {
        Folder::new("root", user_id)
    }

    #[test]
    fn test_folder_new_root() {
        let user_id = Uuid::new_v4();
        let folder: Folder<String> = Folder::new("folder name", user_id);

        assert!(folder.is_root());
        assert_eq!(folder.name(), "folder name");
        assert_eq!(folder.parent_id(), None);
        assert_eq!(folder.path(), "/");
        assert_eq!(folder.files().len(), 0);
    }

    #[test]
    fn test_folder_new_child() {
        let user_id = Uuid::new_v4();
        let root: Folder<String> = make_root(user_id);
        let child = Folder::new_child("child", user_id, &root);

        assert!(!child.is_root());
        assert_eq!(child.name(), "child");
        assert_eq!(child.parent_id(), Some(root.id()));
        assert!(child.path().starts_with('/'));
    }

    #[test]
    fn test_folder_grandchild_path() {
        let user_id = Uuid::new_v4();
        let root: Folder<String> = make_root(user_id);
        let child = Folder::new_child("child", user_id, &root);
        let grandchild = Folder::new_child("grandchild", user_id, &child);

        assert!(grandchild.path().starts_with(child.path()));
        assert_eq!(grandchild.parent_id(), Some(child.id()));
    }

    #[test]
    fn test_depth_root() {
        let user_id = Uuid::new_v4();
        let root: Folder<String> = make_root(user_id);
        assert_eq!(root.depth(), 0);
    }

    #[test]
    fn test_depth_child() {
        let user_id = Uuid::new_v4();
        let root: Folder<String> = make_root(user_id);
        let child = Folder::new_child("child", user_id, &root);
        assert_eq!(child.depth(), 1);
    }

    #[test]
    fn test_depth_grandchild() {
        let user_id = Uuid::new_v4();
        let root: Folder<String> = make_root(user_id);
        let child = Folder::new_child("child", user_id, &root);
        let grandchild = Folder::new_child("grandchild", user_id, &child);
        assert_eq!(grandchild.depth(), 2);
    }

    #[test]
    fn test_rename() {
        let user_id = Uuid::new_v4();
        let mut folder: Folder<String> = make_root(user_id);
        folder.rename("new name");
        assert_eq!(folder.name(), "new name");
    }

    #[test]
    fn test_add_file() {
        let user_id = Uuid::new_v4();
        let mut folder: Folder<String> = make_root(user_id);
        folder.add_file("flashcard".to_string());
        assert_eq!(folder.files().len(), 1);
    }

    #[test]
    fn test_is_ancestor_of_child() {
        let user_id = Uuid::new_v4();
        let root: Folder<String> = make_root(user_id);
        let child = Folder::new_child("child", user_id, &root);
        assert!(root.is_ancestor_of(&child));
    }

    #[test]
    fn test_is_ancestor_of_grandchild() {
        let user_id = Uuid::new_v4();
        let root: Folder<String> = make_root(user_id);
        let child = Folder::new_child("child", user_id, &root);
        let grandchild = Folder::new_child("grandchild", user_id, &child);
        assert!(root.is_ancestor_of(&grandchild));
        assert!(child.is_ancestor_of(&grandchild));
    }

    #[test]
    fn test_is_not_ancestor_of_sibling() {
        let user_id = Uuid::new_v4();
        let root: Folder<String> = make_root(user_id);
        let child_a = Folder::new_child("a", user_id, &root);
        let child_b = Folder::new_child("b", user_id, &root);
        assert!(!child_a.is_ancestor_of(&child_b));
    }

    #[test]
    fn test_is_not_ancestor_of_self() {
        let user_id = Uuid::new_v4();
        let root: Folder<String> = make_root(user_id);
        assert!(!root.is_ancestor_of(&root));
    }

    #[test]
    fn test_can_move_into_sibling() {
        let user_id = Uuid::new_v4();
        let root: Folder<String> = make_root(user_id);
        let child_a = Folder::new_child("a", user_id, &root);
        let child_b = Folder::new_child("b", user_id, &root);
        assert!(child_a.can_move_into(&child_b));
    }

    #[test]
    fn test_cannot_move_into_own_descendant() {
        let user_id = Uuid::new_v4();
        let root: Folder<String> = make_root(user_id);
        let child = Folder::new_child("child", user_id, &root);
        let grandchild = Folder::new_child("grandchild", user_id, &child);
        assert!(!child.can_move_into(&grandchild));
    }

    #[test]
    fn test_cannot_move_into_self() {
        let user_id = Uuid::new_v4();
        let root: Folder<String> = make_root(user_id);
        let child = Folder::new_child("child", user_id, &root);
        assert!(!child.can_move_into(&child));
    }

    #[test]
    fn test_root_cannot_be_moved() {
        let user_id = Uuid::new_v4();
        let root: Folder<String> = make_root(user_id);
        let other: Folder<String> = make_root(user_id);
        assert!(!root.can_move_into(&other));
    }
}
