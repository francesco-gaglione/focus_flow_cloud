use adapters::http::auth::login::{LoginDto, LoginResponseDto};
use adapters::http::users::create_user::CreateUserDto;
use reqwest::StatusCode;

mod common;

#[tokio::test]
async fn test_auth_flow() {
    let ctx = common::setup().await;

    // 1. Create a new user (using Admin account from ctx)
    let new_username = "testuser";
    let new_password = "testpassword123";

    let create_user_dto = CreateUserDto {
        username: new_username.to_string(),
        password: new_password.to_string(),
    };

    let response = ctx
        .client
        .post(format!("{}/api/users", ctx.base_url))
        .json(&create_user_dto)
        .send()
        .await
        .expect("Failed to create user");

    assert_eq!(response.status(), StatusCode::OK);

    // 2. Login as the new user (create new client to avoid carrying over admin token)
    let client = reqwest::Client::new();
    let login_dto = LoginDto {
        username: new_username.to_string(),
        password: new_password.to_string(),
    };

    let response = client
        .post(format!("{}/api/auth/login", ctx.base_url))
        .json(&login_dto)
        .send()
        .await
        .expect("Failed to login");

    assert_eq!(response.status(), StatusCode::OK);

    let login_response: LoginResponseDto = response
        .json()
        .await
        .expect("Failed to deserialize login response");

    assert!(!login_response.token.is_empty());

    // 3. Access protected resource with new token
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        reqwest::header::HeaderValue::from_str(&format!("Bearer {}", login_response.token))
            .unwrap(),
    );
    let user_client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    // Try to get settings (protected route)
    let response = user_client
        .get(format!("{}/api/setting", ctx.base_url))
        .send()
        .await
        .expect("Failed to get settings");

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_login_invalid_credentials() {
    let ctx = common::setup().await;
    let client = reqwest::Client::new();

    let login_dto = LoginDto {
        username: "nonexistent".to_string(),
        password: "wrongpassword".to_string(),
    };

    let response = client
        .post(format!("{}/api/auth/login", ctx.base_url))
        .json(&login_dto)
        .send()
        .await
        .expect("Failed to login");

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_protected_route_without_token() {
    let ctx = common::setup().await;
    let client = reqwest::Client::new(); // No auth headers

    let response = client
        .get(format!("{}/api/setting", ctx.base_url))
        .send()
        .await
        .expect("Failed to request");

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
