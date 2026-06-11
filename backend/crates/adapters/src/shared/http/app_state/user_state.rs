use application::user::use_cases::user::get_user_info::GetUserInfoUseCase;
use application::user::use_cases::user::login_user::LoginUseCase;
use application::user::use_cases::user::refresh_token::RefreshTokenUseCase;
use application::user::use_cases::user::register_user::RegisterUserUseCase;
use application::user::use_cases::user::update_password::UpdateUserPasswordUseCase;
use application::user::use_cases::user::update_user_username::UpdateUserUsernameUseCase;
use application::user::use_cases::user_settings::get_settings::GetSettingsUseCase;
use application::user::use_cases::user_settings::update_setting::UpdateSettingUseCase;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserState {
    pub register_user_uc: Arc<RegisterUserUseCase>,
    pub login_uc: Arc<LoginUseCase>,
    pub refresh_token_uc: Arc<RefreshTokenUseCase>,
    pub update_password_uc: Arc<UpdateUserPasswordUseCase>,
    pub update_user_username_uc: Arc<UpdateUserUsernameUseCase>,
    pub get_user_info_uc: Arc<GetUserInfoUseCase>,
    pub update_user_setting_uc: Arc<UpdateSettingUseCase>,
    pub get_user_settings_uc: Arc<GetSettingsUseCase>,
}
