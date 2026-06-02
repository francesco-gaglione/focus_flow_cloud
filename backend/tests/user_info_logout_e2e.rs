mod common;

use adapters::http::auth::login::{LoginDto, LoginResponseDto};
use adapters::http::auth::logout::LogoutResponseDto;
use adapters::http::users::create_user::CreateUserDto;
use adapters::http::users::get_info::UserInfoResponseDto;
use reqwest::StatusCode;

#[tokio::test]
async fn test_get_user_info_and_logout() {
    let ctx = common::setup().await;

    // 1. Create User
    let username = "info_test_user";
    let password = "Password123!";
    ctx.create_user(&CreateUserDto {
        username: username.to_string(),
        password: password.to_string(),
    })
    .await;

    // 2. Login
    let client = reqwest::Client::new();
    let login_dto = LoginDto {
        username: username.to_string(),
        password: password.to_string(),
    };

    let response = client
        .post(format!("{}/api/auth/login", ctx.base_url))
        .json(&login_dto)
        .send()
        .await
        .expect("Failed to login");

    assert_eq!(response.status(), StatusCode::OK);
    let login_response: LoginResponseDto = response.json().await.unwrap();

    // 3. Get User Info
    let user_client = reqwest::Client::builder()
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", login_response.token))
                    .unwrap(),
            );
            headers
        })
        .build()
        .unwrap();

    let response = user_client
        .get(format!("{}/api/users/me", ctx.base_url))
        .send()
        .await
        .expect("Failed to get user info");

    assert_eq!(response.status(), StatusCode::OK);
    let user_info: UserInfoResponseDto = response.json().await.unwrap();

    assert_eq!(user_info.username, username);
    assert_eq!(user_info.role, "User");

    // 4. Logout
    let response = user_client
        .post(format!("{}/api/auth/logout", ctx.base_url))
        .send()
        .await
        .expect("Failed to logout");

    assert_eq!(response.status(), StatusCode::OK);
    let logout_response: LogoutResponseDto = response.json().await.unwrap();
    assert_eq!(logout_response.message, "Logout successful");
}
