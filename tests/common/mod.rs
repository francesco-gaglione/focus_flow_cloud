use focus_flow_cloud::adapters::http::dto::{
    category_api::create_category::{CreateCategoryDto, CreateCategoryResponseDto},
    session_api::create_manual_session::{CreateManualSessionDto, CreateManualSessionResponseDto},
    task_api::create_task::{CreateTaskDto, CreateTaskResponseDto},
    user_setting_api::{
        get_user_settings::UserSettingsResponseDto, update_setting::UpdateUserSettingDto,
    },
};
use focus_flow_cloud::infra::{
    app::create_app,
    config::AppConfig,
    setup::{init_app_state, init_tracing},
};
use std::sync::Once;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;
use tokio::net::TcpListener;
use tracing::info;

static TRACING: Once = Once::new();

pub struct TestContext {
    pub base_url: String,
    pub client: reqwest::Client,
    #[allow(dead_code)] // Keep container alive
    pub container: testcontainers::ContainerAsync<Postgres>,
}

impl TestContext {
    #[allow(dead_code)]
    pub async fn create_category(&self, dto: &CreateCategoryDto) -> CreateCategoryResponseDto {
        let response = self
            .client
            .post(format!("{}/api/categories", self.base_url))
            .json(dto)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(response.status(), 200);
        response
            .json()
            .await
            .expect("Failed to deserialize response")
    }

    #[allow(dead_code)]
    pub async fn create_task(&self, dto: &CreateTaskDto) -> CreateTaskResponseDto {
        let response = self
            .client
            .post(format!("{}/api/tasks", self.base_url))
            .json(dto)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(response.status(), 200);
        response
            .json()
            .await
            .expect("Failed to deserialize response")
    }

    #[allow(dead_code)]
    pub async fn create_manual_session(
        &self,
        dto: &CreateManualSessionDto,
    ) -> CreateManualSessionResponseDto {
        let response = self
            .client
            .post(format!("{}/api/focus-sessions/manual", self.base_url))
            .json(dto)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(response.status(), 200);

        let response_text = response.text().await.expect("Failed to get response text");

        println!("Response Body: {}", response_text);

        serde_json::from_str(&response_text).expect("Failed to deserialize response from text")
    }

    #[allow(dead_code)]
    pub async fn update_user_setting(&self, dto: &UpdateUserSettingDto) {
        let response = self
            .client
            .patch(format!("{}/api/user-settings", self.base_url))
            .json(dto)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(response.status(), 200);
    }

    #[allow(dead_code)]
    pub async fn get_user_settings(&self) -> UserSettingsResponseDto {
        let response = self
            .client
            .get(format!("{}/api/user-settings", self.base_url))
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(response.status(), 200);
        response
            .json()
            .await
            .expect("Failed to deserialize response")
    }
}

pub async fn setup() -> TestContext {
    // Initialize tracing once
    TRACING.call_once(|| {
        init_tracing();
    });

    // Start Postgres container
    let container = Postgres::default().start().await.unwrap();
    let host_ip = container.get_host().await.unwrap();
    let host_port = container.get_host_port_ipv4(5432).await.unwrap();
    let db_url = format!("postgres://postgres:postgres@{host_ip}:{host_port}/postgres");

    // Setup AppConfig
    let config = AppConfig {
        server_port: 0,
        cors_origin: "*".to_string(),
        database_url: db_url,
    };

    // Initialize App State (this runs migrations)
    let app_state = init_app_state(config).await.unwrap();

    // Create App Router
    let app = create_app(app_state);

    // Start Server
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    TestContext {
        base_url: format!("http://127.0.0.1:{}", port),
        client: reqwest::Client::new(),
        container,
    }
}
