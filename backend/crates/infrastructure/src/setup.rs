use application::shared::use_cases::push_subscription::save_push_subscription::SavePushSubscriptionUseCase;
use application::shared::use_cases::reminder::get_pending_reminders::GetPendingRemindersUseCase;
use application::tasks::use_cases::category::create_category_usecase::CreateCategoryUseCases;
use application::tasks::use_cases::category::delete_categories_usecase::DeleteCategoriesUseCases;
use application::tasks::use_cases::category::get_all_category_usecase::GetAllCategoryUseCases;
use application::tasks::use_cases::category::update_category_usecase::UpdateCategoryUseCases;
use application::tasks::use_cases::focus_session::create_manual_session::CreateManualSessionUseCase;
use application::tasks::use_cases::focus_session::find_sessions_by_filters::FindSessionsByFiltersUseCase;
use application::tasks::use_cases::focus_session::update_focus_session::UpdateFocusSessionUseCase;
use application::tasks::use_cases::pomodoro_state::fetch_user_pomodoro_state::FetchUserPomodoroStateUseCase;
use application::tasks::use_cases::pomodoro_state::init_pomodoro_state::InitPomodoroStateUseCase;
use application::tasks::use_cases::pomodoro_state::start_session::StartSessionUseCase;
use application::tasks::use_cases::pomodoro_state::terminate_session::TerminateSessionUseCase;
use application::tasks::use_cases::pomodoro_state::update_current_session::UpdateSessionUseCase;
use application::tasks::use_cases::pomodoro_state::update_pomodoro_context::UpdatePomodoroContextUseCase;
use application::tasks::use_cases::stats::get_stats::GetStatsUseCase;
use application::tasks::use_cases::task::add_subtask::AddSubTaskUseCase;
use application::tasks::use_cases::task::create_task::CreateTaskUseCase;
use application::tasks::use_cases::task::delete_task::DeleteTaskUseCase;
use application::tasks::use_cases::task::get_tasks::GetTasksUseCase;
use application::tasks::use_cases::task::update_subtask::UpdateSubTaskUseCase;
use application::tasks::use_cases::task::update_task::UpdateTaskUseCase;
use application::user::use_cases::user::get_user_info::GetUserInfoUseCase;
use application::user::use_cases::user::login_user::LoginUseCase;
use application::user::use_cases::user::refresh_token::RefreshTokenUseCase;
use application::user::use_cases::user::register_user::RegisterUserUseCase;
use application::user::use_cases::user::update_password::UpdateUserPasswordUseCase;
use application::user::use_cases::user::update_user_username::UpdateUserUsernameUseCase;
use application::user::use_cases::user_settings::get_settings::GetSettingsUseCase;
use application::user::use_cases::user_settings::update_setting::UpdateSettingUseCase;
use opentelemetry::global;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::Resource;
use tracing::info;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::Registry;

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
use adapters::persistence::persistence_impl::pomodoro_state_in_memory_impl::PomodoroStateInMermoryImpl;
use adapters::persistence::persistence_impl::reminder_worker_port_impl::{
    spawn_reminder_worker, ReminderWorkerPortImpl,
};
use application::tasks::use_cases::pomodoro_state::pause_session::PauseSessionUseCase;
use application::user::traits::password_hasher::PasswordHasher;
use application::user::traits::user_persistence::UserPersistence;
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

pub async fn init_app_state(
    config: AppConfig,
    version: String,
) -> Result<AppState, Box<dyn std::error::Error>> {
    let persistence = postgres_persistence(&config.database_url).await;
    run_migrations(&persistence.pool).await;
    let postgres_arc = Arc::new(persistence);

    let sqlx_pool = sqlx::PgPool::connect(&config.database_url)
        .await
        .expect("Failed to create sqlx pool for apalis");
    let reminder_worker = Arc::new(ReminderWorkerPortImpl::new(sqlx_pool.clone()));

    info!("Spawning reminder worker");
    spawn_reminder_worker(
        &sqlx_pool,
        postgres_arc.clone(),
        postgres_arc.clone(),
        config.vapid_private_key.clone(),
    )
    .await;
    info!("Reminder worker spawned");

    let pomodoro_state_arc = Arc::new(PomodoroStateInMermoryImpl::new());

    // Password Hasher
    let argon_hasher = Arc::new(Argon2Hasher::new());

    // Policy
    let password_policy = Arc::new(PasswordPolicyImpl::new());

    // Pomodoro state use cases
    let init_pomodoro_state_uc =
        Arc::new(InitPomodoroStateUseCase::new(pomodoro_state_arc.clone()));
    let pause_pomo_session_uc = Arc::new(PauseSessionUseCase::new(
        pomodoro_state_arc.clone(),
        postgres_arc.clone(),
    ));
    let fetch_pomo_session_uc = Arc::new(FetchUserPomodoroStateUseCase::new(
        pomodoro_state_arc.clone(),
    ));
    let update_pomodoro_context_uc = Arc::new(UpdatePomodoroContextUseCase::new(
        pomodoro_state_arc.clone(),
    ));
    let start_session_uc = Arc::new(StartSessionUseCase::new(
        pomodoro_state_arc.clone(),
        postgres_arc.clone(),
    ));
    let terminate_session_uc = Arc::new(TerminateSessionUseCase::new(
        pomodoro_state_arc.clone(),
        postgres_arc.clone(),
    ));
    let update_current_session_uc = Arc::new(UpdateSessionUseCase::new(pomodoro_state_arc.clone()));

    // Category Use Cases
    let create_category_uc = Arc::new(CreateCategoryUseCases::new(postgres_arc.clone()));
    let delete_categories_uc = Arc::new(DeleteCategoriesUseCases::new(postgres_arc.clone()));
    let get_all_category_uc = Arc::new(GetAllCategoryUseCases::new(postgres_arc.clone()));
    let update_category_uc = Arc::new(UpdateCategoryUseCases::new(postgres_arc.clone()));

    // Task Use Cases
    let create_task_uc = Arc::new(CreateTaskUseCase::new(
        postgres_arc.clone(),
        postgres_arc.clone(),
        reminder_worker,
    ));
    let get_tasks_uc = Arc::new(GetTasksUseCase::new(
        postgres_arc.clone(),
        postgres_arc.clone(),
    ));
    let delete_tasks_uc = Arc::new(DeleteTaskUseCase::new(postgres_arc.clone()));
    let save_push_subscription_uc =
        Arc::new(SavePushSubscriptionUseCase::new(postgres_arc.clone()));
    let get_pending_reminders_uc = Arc::new(GetPendingRemindersUseCase::new(postgres_arc.clone()));
    let update_task_uc = Arc::new(UpdateTaskUseCase::new(postgres_arc.clone()));
    let update_subtask_uc = Arc::new(UpdateSubTaskUseCase::new(postgres_arc.clone()));
    let add_subtask_uc = Arc::new(AddSubTaskUseCase::new(postgres_arc.clone()));

    // Stats Use Cases
    let get_stats_uc = Arc::new(GetStatsUseCase::new(
        postgres_arc.clone(),
        postgres_arc.clone(),
        postgres_arc.clone(),
    ));

    // Focus Session Use Cases
    let create_manual_session_uc = Arc::new(CreateManualSessionUseCase::new(postgres_arc.clone()));
    let update_focus_session_uc = Arc::new(UpdateFocusSessionUseCase::new(postgres_arc.clone()));
    let find_sessions_by_filters_uc =
        Arc::new(FindSessionsByFiltersUseCase::new(postgres_arc.clone()));

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
        use domain::user::entities::{user::User, user_role::UserRole};
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
        config: config.clone(),
        init_pomodoro_state_uc,
        pause_pomo_session_uc,
        fetch_pomo_session_uc,
        update_pomodoro_context_uc,
        start_session_uc,
        terminate_session_uc,
        update_current_session_uc,
        get_stats_uc,
        get_all_category_uc,
        create_category_uc,
        delete_categories_uc,
        update_category_uc,
        get_tasks_uc,
        create_task_uc,
        delete_tasks_uc,
        update_task_uc,
        update_subtask_uc,
        add_subtask_uc,
        create_manual_session_uc,
        update_focus_session_uc,
        find_sessions_by_filters_uc,
        update_user_setting_uc,
        get_user_settings_uc,
        register_user_uc,
        login_uc,
        refresh_token_uc,
        update_password_uc,
        update_user_username_uc,
        get_user_info_uc,
        save_push_subscription_uc,
        get_pending_reminders_uc,
        token_service,
        version,
    })
}

pub fn init_tracing<Sink>(sink: Sink)
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        "focus_flow_cloud=debug,api=debug,domain=debug,infrastructure=debug,application=debug,tower_http=info,axum=info,info".into()
    });

    let formatting_layer = BunyanFormattingLayer::new("focus_flow_cloud".to_string(), sink);

    let otel_layer = std::env::var("OTLP_ENDPOINT").ok().map(|endpoint| {
        let exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_tonic()
            .with_endpoint(endpoint)
            .build()
            .expect("Failed to build OTLP span exporter");

        let provider = opentelemetry_sdk::trace::TracerProvider::builder()
            .with_batch_exporter(exporter, opentelemetry_sdk::runtime::Tokio)
            .with_resource(Resource::new(vec![opentelemetry::KeyValue::new(
                "service.name",
                "focus_flow_cloud",
            )]))
            .build();

        global::set_tracer_provider(provider.clone());
        let tracer = provider.tracer("focus_flow_cloud");
        tracing_opentelemetry::layer().with_tracer(tracer)
    });

    LogTracer::init().expect("Failed to set logger");

    let registry = Registry::default()
        .with(filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .with(otel_layer);

    set_global_default(registry).expect("Failed to set subscriber");
}
