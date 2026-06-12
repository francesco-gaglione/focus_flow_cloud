use adapters::shared::http::app_state::user_state::UserState;
use adapters::shared::persistence::PostgresPersistence;
use adapters::user::auth::password_policy_impl::PasswordPolicyImpl;
use application::user::use_cases::user::get_user_info::GetUserInfoUseCase;
use application::user::use_cases::user::login_user::LoginUseCase;
use application::user::use_cases::user::refresh_token::RefreshTokenUseCase;
use application::user::use_cases::user::register_user::RegisterUserUseCase;
use application::user::use_cases::user::update_password::UpdateUserPasswordUseCase;
use application::user::use_cases::user::update_user_username::UpdateUserUsernameUseCase;
use application::user::use_cases::user_settings::get_settings::GetSettingsUseCase;
use application::user::use_cases::user_settings::update_setting::UpdateSettingUseCase;
use crate::crypto::password_hasher::Argon2Hasher;
use crate::services::jwt_service::JwtService;
use std::sync::Arc;

pub fn build_user_state(
    postgres: Arc<PostgresPersistence>,
    argon_hasher: Arc<Argon2Hasher>,
    password_policy: Arc<PasswordPolicyImpl>,
    token_service: Arc<JwtService>,
) -> UserState {
    UserState {
        register_user_uc: Arc::new(RegisterUserUseCase::new(
            argon_hasher.clone(),
            postgres.clone(),
            password_policy.clone(),
        )),
        login_uc: Arc::new(LoginUseCase::new(
            postgres.clone(),
            argon_hasher.clone(),
            token_service.clone(),
        )),
        refresh_token_uc: Arc::new(RefreshTokenUseCase::new(
            postgres.clone(),
            token_service.clone(),
        )),
        update_password_uc: Arc::new(UpdateUserPasswordUseCase::new(
            argon_hasher.clone(),
            postgres.clone(),
            password_policy.clone(),
        )),
        update_user_username_uc: Arc::new(UpdateUserUsernameUseCase::new(postgres.clone())),
        get_user_info_uc: Arc::new(GetUserInfoUseCase::new(postgres.clone())),
        update_user_setting_uc: Arc::new(UpdateSettingUseCase::new(postgres.clone())),
        get_user_settings_uc: Arc::new(GetSettingsUseCase::new(postgres.clone())),
    }
}
