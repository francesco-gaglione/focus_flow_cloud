use crate::clients::user_client::update_password;

pub async fn update_password_uc(old_password: &str, new_password: &str) -> Result<(), String> {
    update_password(old_password, new_password)
        .await
        .map_err(|e| e.to_string())
}
