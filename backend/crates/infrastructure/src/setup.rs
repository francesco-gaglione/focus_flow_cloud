use application::use_cases::focus_session::update_focus_session::UpdateFocusSessionUseCase;
use application::use_cases::task::complete_task::CompleteTaskUseCase;
use application::use_cases::task::get_scheduled_tasks::GetScheduledTasksUseCase;
use application::use_cases::task::get_tasks::GetTasksUseCase;
use application::use_cases::user::get_user_info::GetUserInfoUseCase;
use application::use_cases::user::refresh_token::RefreshTokenUseCase;
use application::use_cases::user::register_user::RegisterUserUseCase;
use application::use_cases::user::update_password::UpdateUserPasswordUseCase;
use application::use_cases::user::update_user_username::UpdateUserUsernameUseCase;
use application::use_cases::user_settings::get_settings::GetSettingsUseCase;
use application::use_cases::user_settings::update_setting::UpdateSettingUseCase;
use application::use_cases::{
    category::{
        create_category_usecase::CreateCategoryUseCases,
        delete_categories_usecase::DeleteCategoriesUseCases,
        delete_category_usecase::DeleteCategoryUseCases,
        get_category_and_task_usecase::GetCategoryAndTaskUseCases,
        get_category_usecase::GetCategoryUseCases, update_category_usecase::UpdateCategoryUseCases,
    },
    focus_session::{
        create_manual_session::CreateManualSessionUseCase, create_session::CreateSessionUseCase,
        find_sessions_by_filters::FindSessionsByFiltersUseCase,
    },
    stats::calculate_stats_by_period::CalculateStatsByPeriodUseCase,
    task::{
        create_task::CreateTaskUseCase, delete_tasks::DeleteTasksUseCase,
        orphan_tasks::OrphanTasksUseCase, update_task::UpdateTaskUseCase,
    },
    user::login_user::LoginUseCase,
};

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::crypto::password_hasher::Argon2Hasher;
use crate::database::run_migrations;
use crate::services::jwt_service::JwtService;
use adapters::auth::password_policy_impl::PasswordPolicyImpl;
use adapters::config::AppConfig;
use adapters::http::app_state::AppState;
use adapters::persistence::persistence_impl::persistence::postgres_persistence;
use application::auth_traits::password_hasher::PasswordHasher;
use application::persistence_traits::user_persistence::UserPersistence;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub async fn init_app_state(
    config: AppConfig,
    version: String,
) -> Result<AppState, Box<dyn std::error::Error>> {
    let persistence = postgres_persistence(&config.database_url).await;
    run_migrations(&persistence.pool).await;
    let postgres_arc = Arc::new(persistence);

    // Password Hasher
    let argon_hasher = Arc::new(Argon2Hasher::new());

    // Policy
    let password_policy = Arc::new(PasswordPolicyImpl::new());

    // Category Use Cases
    let create_category_uc = Arc::new(CreateCategoryUseCases::new(postgres_arc.clone()));
    let delete_categories_uc = Arc::new(DeleteCategoriesUseCases::new(postgres_arc.clone()));
    let delete_category_uc = Arc::new(DeleteCategoryUseCases::new(postgres_arc.clone()));
    let get_category_and_task_uc = Arc::new(GetCategoryAndTaskUseCases::new(
        postgres_arc.clone(),
        postgres_arc.clone(),
    ));
    let get_category_uc = Arc::new(GetCategoryUseCases::new(postgres_arc.clone()));
    let update_category_uc = Arc::new(UpdateCategoryUseCases::new(postgres_arc.clone()));

    // Task Use Cases
    let create_task_uc = Arc::new(CreateTaskUseCase::new(postgres_arc.clone()));
    let get_tasks_uc = Arc::new(GetTasksUseCase::new(postgres_arc.clone()));
    let delete_tasks_uc = Arc::new(DeleteTasksUseCase::new(postgres_arc.clone()));
    let orphan_tasks_uc = Arc::new(OrphanTasksUseCase::new(postgres_arc.clone()));
    let update_task_uc = Arc::new(UpdateTaskUseCase::new(postgres_arc.clone()));
    let complete_task_uc = Arc::new(CompleteTaskUseCase::new(postgres_arc.clone()));
    let get_scheduled_task_uc = Arc::new(GetScheduledTasksUseCase::new(postgres_arc.clone()));

    // Focus Session Use Cases
    let create_manual_session_uc = Arc::new(CreateManualSessionUseCase::new(postgres_arc.clone()));
    let create_session_uc = Arc::new(CreateSessionUseCase::new(postgres_arc.clone()));
    let update_focus_session_uc = Arc::new(UpdateFocusSessionUseCase::new(postgres_arc.clone()));
    let find_sessions_by_filters_uc =
        Arc::new(FindSessionsByFiltersUseCase::new(postgres_arc.clone()));

    // Stats Use Cases
    let calculate_stats_by_period_uc = Arc::new(CalculateStatsByPeriodUseCase::new(
        postgres_arc.clone(),
        postgres_arc.clone(),
        postgres_arc.clone(),
    ));

    // User Setting Use Cases
    let get_user_settings_uc = Arc::new(GetSettingsUseCase::new(postgres_arc.clone()));
    let update_user_setting_uc = Arc::new(UpdateSettingUseCase::new(postgres_arc.clone()));

    // Token Service
    let token_service = Arc::new(JwtService::new(config.jwt_secret.clone()));

    // User Use Cases
    let register_user_uc = Arc::new(RegisterUserUseCase::new(
        argon_hasher.clone(),
        postgres_arc.clone(),
        password_policy.clone(),
    ));

    let login_uc = Arc::new(LoginUseCase::new(
        postgres_arc.clone(),
        argon_hasher.clone(),
        token_service.clone(),
    ));

    let refresh_token_uc = Arc::new(RefreshTokenUseCase::new(
        postgres_arc.clone(),
        token_service.clone(),
    ));

    let update_password_uc = Arc::new(UpdateUserPasswordUseCase::new(
        argon_hasher.clone(),
        postgres_arc.clone(),
        password_policy.clone(),
    ));

    let update_user_username_uc = Arc::new(UpdateUserUsernameUseCase::new(postgres_arc.clone()));

    let get_user_info_uc = Arc::new(GetUserInfoUseCase::new(postgres_arc.clone()));

    // Seed Admin User
    if let (Some(username), Some(password)) = (&config.admin_username, &config.admin_password) {
        use domain::entities::{user::User, user_role::UserRole};
        use tracing::{error, info};

        info!("Checking for admin user: {}", username);

        match postgres_arc.find_user_by_username(username).await {
            Ok(_) => {
                info!(
                    "Admin user '{}' already exists. Skipping creation.",
                    username
                );
            }
            Err(_) => {
                info!("Admin user '{}' not found. Creating...", username);
                match argon_hasher.hash_password(password) {
                    Ok(hashed_password) => {
                        let admin_user =
                            User::new(username.clone(), hashed_password, UserRole::Admin);

                        match postgres_arc.create_user(admin_user).await {
                            Ok(id) => info!(
                                "Successfully created admin user '{}' with ID: {}",
                                username, id
                            ),
                            Err(e) => error!("Failed to create admin user: {:?}", e),
                        }
                    }
                    Err(e) => error!("Failed to hash admin password: {:?}", e),
                }
            }
        }
    }

    Ok(AppState {
        ws_clients: Arc::new(RwLock::new(HashMap::new())),
        pomodoro_states: Arc::new(RwLock::new(HashMap::new())),
        config: config.clone(),
        create_category_uc,
        delete_categories_uc,
        delete_category_uc,
        get_category_and_task_uc,
        get_category_uc,
        update_category_uc,
        get_tasks_uc,
        create_task_uc,
        delete_tasks_uc,
        orphan_tasks_uc,
        update_task_uc,
        complete_task_uc,
        get_scheduled_task_uc,
        create_manual_session_uc,
        update_focus_session_uc,
        create_session_uc,
        find_sessions_by_filters_uc,
        calculate_stats_by_period_uc,
        update_user_setting_uc,
        get_user_settings_uc,
        register_user_uc,
        login_uc,
        refresh_token_uc,
        update_password_uc,
        update_user_username_uc,
        get_user_info_uc,
        token_service,
        version,
    })
}

pub fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        // Default to info, but enable debug for our app
        // tower_http=info reduces noise from every single request detail if needed, but debug is good for dev
        "focus_flow_cloud=debug,api=debug,domain=debug,infrastructure=debug,application=debug,tower_http=info,axum=info,info".into()
    });

    let registry = tracing_subscriber::registry().with(filter);

    let app_env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());

    if app_env == "production" {
        registry
            .with(tracing_subscriber::fmt::layer().json())
            .init();
    } else {
        registry
            .with(
                tracing_subscriber::fmt::layer()
                    .pretty()
                    .with_target(true)
                    .with_thread_ids(true)
                    .with_file(false)
                    .with_line_number(false),
            )
            .init();
    }
}
