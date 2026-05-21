use crate::clients::user_client::update_username;

pub async fn update_username_uc(new_username: &str) -> Result<(), String> {
    update_username(new_username)
        .await
        .map_err(|e| e.to_string())
}
