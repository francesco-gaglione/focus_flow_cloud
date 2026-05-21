use crate::clients::user_client::create_user;

pub async fn create_user_uc(username: &str, password: &str) -> Result<(), String> {
    create_user(username, password)
        .await
        .map_err(|e| e.to_string())
}
