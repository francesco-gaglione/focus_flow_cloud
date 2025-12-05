use api::adapters::http::dto::user_setting_api::update_setting::UpdateUserSettingDto;
use common::setup;

mod common;

#[tokio::test]
async fn test_user_settings_flow() {
    let ctx = setup().await;

    // 1. Get Initial Settings (Should be empty or contain defaults, but we start fresh DB)
    let initial_settings = ctx.get_user_settings().await;
    // Assuming fresh DB might have no settings or some defaults.
    // For now, let's assume we can add any setting.
    let _initial_count = initial_settings.settings.len();

    // 2. Update a Setting (Add new)
    let setting_key = "theme";
    let setting_value = "dark";

    ctx.update_user_setting(&UpdateUserSettingDto {
        key: setting_key.to_string(),
        value: setting_value.to_string(),
    })
    .await;

    // 3. Verify Update
    let updated_settings = ctx.get_user_settings().await;
    let theme_setting = updated_settings
        .settings
        .iter()
        .find(|s| s.key == setting_key);

    assert!(theme_setting.is_some());
    assert_eq!(theme_setting.unwrap().value, setting_value);

    // 4. Update Existing Setting
    let new_value = "light";
    ctx.update_user_setting(&UpdateUserSettingDto {
        key: setting_key.to_string(),
        value: new_value.to_string(),
    })
    .await;

    // 5. Verify Second Update
    let final_settings = ctx.get_user_settings().await;
    let theme_setting_final = final_settings
        .settings
        .iter()
        .find(|s| s.key == setting_key);

    assert!(theme_setting_final.is_some());
    assert_eq!(theme_setting_final.unwrap().value, new_value);

    // Count should be same as after first insert (if it was a new insert) or +1 from initial if it wasn't there
    // Since "theme" might not be in initial, let's check count relative to initial
    // Actually, if "theme" was not there, count increased by 1.
    // If it was there, count stayed same.
    // Let's checks if we added a completely new one.

    let another_key = "notifications";
    let another_value = "enabled";

    ctx.update_user_setting(&UpdateUserSettingDto {
        key: another_key.to_string(),
        value: another_value.to_string(),
    })
    .await;

    let settings_with_two = ctx.get_user_settings().await;
    assert!(settings_with_two
        .settings
        .iter()
        .any(|s| s.key == another_key && s.value == another_value));
}
