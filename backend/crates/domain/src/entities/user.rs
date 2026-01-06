use uuid::Uuid;

use crate::entities::user_role::UserRole;

#[derive(Debug, Clone)]
pub struct User {
    id: Uuid,
    username: String,
    hashed_password: String,
    role: UserRole,
}

impl User {
    pub fn new(username: String, hashed_password: String, role: UserRole) -> Self {
        User {
            id: Uuid::new_v4(),
            username,
            hashed_password,
            role,
        }
    }

    pub fn reconstitute(
        id: Uuid,
        username: String,
        hashed_password: String,
        role: UserRole,
    ) -> Self {
        User {
            id,
            username,
            hashed_password,
            role,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn hashed_password(&self) -> &str {
        &self.hashed_password
    }

    pub fn update_username(&mut self, new_username: String) {
        self.username = new_username;
    }

    pub fn update_password(&mut self, new_hashed_password: String) {
        self.hashed_password = new_hashed_password;
    }

    pub fn role(&self) -> &UserRole {
        &self.role
    }

    pub fn update_role(&mut self, new_role: UserRole) {
        self.role = new_role;
    }

    pub fn is_admin(&self) -> bool {
        self.role == UserRole::Admin
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::user_role::UserRole;

    #[test]
    fn test_new_user() {
        let user = User::new(
            "test_user".to_string(),
            "hashed_password".to_string(),
            UserRole::User,
        );

        assert_eq!(user.id().to_string().len(), 36);
        assert_eq!(user.username(), "test_user");
        assert_eq!(user.hashed_password(), "hashed_password");
        assert_eq!(user.role(), &UserRole::User);
        assert!(!user.is_admin());
    }

    #[test]
    fn test_reconstitute_user() {
        let id = Uuid::new_v4();
        let user = User::reconstitute(
            id,
            "test_user".to_string(),
            "hashed_password".to_string(),
            UserRole::Admin,
        );

        assert_eq!(user.id(), id);
        assert_eq!(user.username(), "test_user");
        assert_eq!(user.hashed_password(), "hashed_password");
        assert_eq!(user.role(), &UserRole::Admin);
        assert!(user.is_admin());
    }

    #[test]
    fn test_update_user_details() {
        let mut user = User::new(
            "test_user".to_string(),
            "hashed_password".to_string(),
            UserRole::User,
        );

        user.update_username("new_username".to_string());
        user.update_password("new_hashed_password".to_string());

        assert_eq!(user.username(), "new_username");
        assert_eq!(user.hashed_password(), "new_hashed_password");
        assert_eq!(user.role(), &UserRole::User);
    }

    #[test]
    fn test_update_role() {
        let mut user = User::new("test_user".to_string(), "pass".to_string(), UserRole::User);

        assert_eq!(user.role(), &UserRole::User);
        assert!(!user.is_admin());

        user.update_role(UserRole::Admin);

        assert_eq!(user.role(), &UserRole::Admin);
        assert!(user.is_admin());
    }
}
